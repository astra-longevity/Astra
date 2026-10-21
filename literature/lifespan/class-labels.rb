# Builds class-labels.yaml from class-labels-rules.md (applied as code below), the alias table,
# a committed ChEMBL 37 snapshot and pinned MeSH 2026 files. No label is set by hand.
#
#   ruby class-labels.rb --fetch          # once: ChEMBL REST -> class-labels-chembl37.json
#   ruby class-labels.rb --pin DIR        # once: sha256 of DIR/pa2026.xml, DIR/desc2026.xml -> sources
#   MESH_DIR=DIR ruby class-labels.rb     # build class-labels.yaml (offline, deterministic)
#   MESH_DIR=DIR ruby class-labels.rb --check   # exit 1 if the committed yaml differs from a rebuild
require "csv"
require "json"
require "yaml"
require "digest"
require "date"
require "net/http"
require "uri"

DIR = __dir__
REPO = File.expand_path("../..", DIR)
SPLIT = File.join(REPO, "research", "metric", "v0", "split.csv")
# Train mode (rules amendment 4, §1b): ITP train compounds, own snapshot + output, shared MeSH.
TRAIN = !ENV["TRAIN"].to_s.empty?
ITP_DIR = File.join(DIR, "itp")
SNAP = File.join(DIR, TRAIN ? "class-labels-train-chembl37.json" : "class-labels-chembl37.json")
SOURCES = File.join(DIR, TRAIN ? "class-labels-train-sources.yaml" : "class-labels-sources.yaml")
OUT = File.join(DIR, TRAIN ? "train-classes.yaml" : "class-labels.yaml")
SPLIT_OUT = File.join(DIR, "class-labels.yaml") # sealed split labels, read by the overlap check
ALIASES = YAML.safe_load(File.read(File.join(DIR, "class-labels-aliases.yaml")))["aliases"]
# Train-name canonicalisation (rules amendment 4, §1b): ITP spelling -> canonical split/alias key.
TRAIN_NAMES = TRAIN ? (YAML.safe_load(File.read(File.join(DIR, "class-labels-train-names.yaml")))["canonical"] || {}) : {}
# The 5 compounds present in both the split and the train snapshot (rules §8).
OVERLAP = ["canagliflozin", "captopril", "metformin", "nicotinamide riboside", "rapamycin"].freeze
MESH_BASE = "https://nlmpubs.nlm.nih.gov/projects/mesh/MESH_FILES/xmlmesh/"
MESH_FILES = %w[pa2026.xml desc2026.xml].freeze
CHEMBL = "https://www.ebi.ac.uk/chembl/api/data/"

# Rules section 6: ChEMBL target -> class. A usable target not listed here stops the build.
TARGET_CLASS = {
  "CHEMBL2096665" => "PDK inhibitor", "CHEMBL3884" => "SGLT2 inhibitor",
  "CHEMBL1808" => "ACE inhibitor", "CHEMBL2111289" => "MEK inhibitor",
  "CHEMBL3587" => "MEK inhibitor", "CHEMBL2964" => "MEK inhibitor",
  "CHEMBL2039" => "MAO-B inhibitor", "CHEMBL1827" => "PDE5 inhibitor",
  "CHEMBL4005" => "PI3K-alpha inhibitor", "CHEMBL284" => "DPP-4 inhibitor",
  "CHEMBL1250417" => "HCN channel blocker", "CHEMBL613897" => "haem polymerisation inhibitor",
  # Rules amendment 4 (train snapshot targets): mechanical, from target pref_name + action_type.
  "CHEMBL2045" => "alpha-amylase inhibitor", "CHEMBL2074" => "alpha-glucosidase inhibitor",
  "CHEMBL2094253" => "cyclooxygenase inhibitor", "CHEMBL227" => "angiotensin II receptor antagonist",
  "CHEMBL2111348" => "soluble guanylate cyclase inhibitor", "CHEMBL2363135" => "bacterial ribosome inhibitor",
  "CHEMBL402" => "HMG-CoA reductase inhibitor", "CHEMBL2047" => "FXR agonist"
}.freeze
# Rules section 4: per-compound calls (class, basis). NDGA and chloroquine follow section 3.
CALLS = {
  "rapamycin" => ["mTORC1 inhibitor", "A50 #141 pt 1: own ChEMBL mechanism record (CHEMBL413) + IUPHAR ligand 6031 (login-only since 2026-10-20); no FKBP1A table row"],
  "metformin" => ["multi-target", "A28 (G158): ChEMBL wins; complex I + mGPD, unranked"],
  "cinnarizine" => ["multi-target", "A28 (G158): ChEMBL wins; L-type Ca channel + H1, unranked"]
}.freeze
UNUSABLE_MOA = /\A(unknown|supplement)/i
AGEING = /aging|ageing|longevity|senescen|geroprotect/i
PA_ROOT = "D27.505.519"
FIXED_VOCAB = ["multi-target", "no defined target", "unclassified"].freeze

def norm(s)
  s.to_s.downcase.gsub("α", "alpha").gsub("β", "beta").strip
end

# Train mode: map an ITP spelling to its canonical split/alias key before any lookup (§1b).
def canonical(compound)
  TRAIN_NAMES[compound] || compound
end

def names_for(compound)
  key = canonical(compound)
  ([key] + ALIASES.fetch(key, [])).map { |n| norm(n) }.uniq
end

def compounds
  if TRAIN
    Dir[File.join(ITP_DIR, "*.yaml")].sort.map { |f| YAML.safe_load(File.read(f), [Date, Time, Symbol])["compound"] }
      .compact.uniq.sort_by(&:downcase)
  else
    CSV.read(SPLIT, headers: true).map { |r| r["compound"] }.compact.uniq.sort_by(&:downcase)
  end
end

# ---------- fetch (online, once) ----------
def get(path)
  sleep 0.25
  res = Net::HTTP.get_response(URI(CHEMBL + path))
  raise "ChEMBL #{res.code} for #{path}" unless res.is_a?(Net::HTTPSuccess)
  JSON.parse(res.body)
end

def fetch
  status = get("status.json")
  raise "expected ChEMBL_37, got #{status['chembl_db_version']}" unless status["chembl_db_version"] == "ChEMBL_37"
  targets = {}
  data = compounds.map do |c|
    parents = {}
    names_for(c).each do |n|
      q = URI.encode_www_form_component(n)
      mols = get("molecule.json?pref_name__iexact=#{q}&limit=20")["molecules"] +
             get("molecule.json?molecule_synonyms__molecule_synonym__iexact=#{q}&limit=20")["molecules"]
      mols.each do |m|
        p = m.dig("molecule_hierarchy", "parent_chembl_id") || m["molecule_chembl_id"]
        (parents[p] ||= []) << n
      end
    end
    entries = parents.keys.sort.map do |p|
      forms = ([p] + get("molecule_form.json?parent_chembl_id=#{p}&limit=100")["molecule_forms"].map { |f| f["molecule_chembl_id"] }).uniq.sort
      mech = forms.flat_map do |f|
        get("mechanism.json?molecule_chembl_id=#{f}&limit=100")["mechanisms"].map do |x|
          { "form" => f, "mechanism_of_action" => x["mechanism_of_action"], "action_type" => x["action_type"],
            "target" => x["target_chembl_id"], "mec_id" => x["mec_id"] }
        end
      end.sort_by { |x| [x["form"], x["mec_id"].to_i] }
      mech.each { |x| targets[x["target"]] ||= nil if x["target"] }
      { "parent" => p, "matched_names" => parents[p].uniq.sort, "forms" => forms, "mechanisms" => mech }
    end
    { "compound" => c, "molecules" => entries }
  end
  targets.keys.sort.each { |t| targets[t] = get("target/#{t}.json")["pref_name"] }
  snap = { "release" => status["chembl_db_version"], "release_date" => status["chembl_release_date"],
           "fetched" => Date.today.iso8601, "endpoints" => %w[status molecule molecule_form mechanism target],
           "targets" => targets, "compounds" => data }
  File.write(SNAP, JSON.pretty_generate(snap) + "\n")
  puts "wrote #{SNAP}: #{data.size} compounds, #{targets.size} targets"
end

def pin(dir)
  doc = {
    "chembl" => { "file" => File.basename(SNAP), "sha256" => Digest::SHA256.file(SNAP).hexdigest,
                  "release" => JSON.parse(File.read(SNAP))["release"] },
    "mesh" => MESH_FILES.map do |f|
      { "file" => f, "url" => MESH_BASE + f, "downloaded" => Date.today.iso8601,
        "sha256" => Digest::SHA256.file(File.join(dir, f)).hexdigest, "committed" => false }
    end
  }
  File.write(SOURCES, "# Pinned sources for class-labels.rb (rules amendment 1, section 5). Written by --pin.\n" + doc.to_yaml)
  puts "wrote #{SOURCES}"
end

# ---------- MeSH parsing (line based; the files are NLM's pretty-printed XML) ----------
def parse_desc(path)
  desc = {}
  cur = nil
  ctx = nil
  File.foreach(path) do |l|
    if l.include?("<DescriptorRecord ")
      cur = { "trees" => [], "terms" => [] }
      ctx = :head
    elsif cur && ctx == :head && (m = l.match(%r{<DescriptorUI>(D\d+)</DescriptorUI>}))
      cur["ui"] = m[1]
      ctx = :name
    elsif cur && ctx == :name && (m = l.match(%r{<String>(.*)</String>}))
      cur["name"] = m[1]
      ctx = :body
    elsif cur && (m = l.match(%r{<TreeNumber>(.*)</TreeNumber>}))
      cur["trees"] << m[1]
    elsif cur && l.include?("<Term ")
      ctx = :term
    elsif cur && ctx == :term && (m = l.match(%r{<String>(.*)</String>}))
      cur["terms"] << m[1]
      ctx = :body
    elsif cur && l.include?("</DescriptorRecord>")
      desc[cur["ui"]] = cur
      cur = nil
    end
  end
  desc
end

def parse_pa(path)
  pa = Hash.new { |h, k| h[k] = [] }
  pa_ui = nil
  in_sub = false
  sub_ui = nil
  File.foreach(path) do |l|
    if l.include?("<DescriptorReferredTo>")
      pa_ui = :next
    elsif pa_ui == :next && (m = l.match(%r{<DescriptorUI>(D\d+)</DescriptorUI>}))
      pa_ui = m[1]
    elsif l.include?("<Substance>")
      in_sub = true
    elsif in_sub && (m = l.match(%r{<RecordUI>(\w+)</RecordUI>}))
      sub_ui = m[1]
    elsif in_sub && (m = l.match(%r{<String>(.*)</String>}))
      pa[pa_ui] << [sub_ui, m[1]]
      in_sub = false
    end
  end
  pa
end

# ---------- build ----------
def usable_records(entry)
  entry["molecules"].flat_map { |m| m["mechanisms"] }.select do |x|
    x["target"] && !x["mechanism_of_action"].to_s.match?(UNUSABLE_MOA)
  end
end

def chembl_cites(records, snap)
  records.map { |x| "ChEMBL 37 #{x['form']}: #{x['mechanism_of_action']} (#{x['target']}, #{snap['targets'][x['target']]})" }.uniq
end

def class_from_chembl(entry, snap, flags)
  usable = usable_records(entry)
  return [nil, []] if usable.empty?
  tids = usable.map { |x| x["target"] }.uniq.sort
  cites = chembl_cites(usable, snap)
  return [TARGET_CLASS.fetch(tids.first) { raise "unmapped ChEMBL target #{tids.first} for #{entry['compound']}" }, cites] if tids.size == 1
  mapped = tids.map { |t| TARGET_CLASS[t] || t }.uniq
  return [mapped.first, cites] if mapped.size == 1
  flags << "multi-target by rules section 3 (#{tids.join(', ')}, unranked)"
  ["multi-target", cites]
end

def class_from_mesh(compound, desc, pa_index, flags)
  names = names_for(compound)
  pas = pa_index.select { |_, subs| subs.any? { |s| names.include?(s) } }.keys
  return [nil, []] if pas.empty?
  adm = pas.select do |ui|
    d = desc[ui]
    d && d["trees"].any? { |t| t.start_with?("#{PA_ROOT}.") } && !d["name"].match?(AGEING)
  end
  ignored = (pas - adm).map { |ui| desc.dig(ui, "name") || ui }.sort
  flags << "MeSH PA outside #{PA_ROOT} ignored: #{ignored.join('; ')}" unless ignored.empty?
  return [nil, []] if adm.empty?
  trees = ->(ui) { desc[ui]["trees"].select { |t| t.start_with?("#{PA_ROOT}.") } }
  specific = adm.reject do |a|
    adm.any? { |b| b != a && trees.(b).any? { |tb| trees.(a).any? { |ta| tb.start_with?("#{ta}.") } } }
  end.sort
  cites = specific.map { |ui| "MeSH #{ui} #{desc[ui]['name']} (https://meshb.nlm.nih.gov/record/ui?ui=#{ui})" }
  return [desc[specific.first]["name"], cites] if specific.size == 1
  flags << "multi-target by rules section 5 (#{specific.size} admissible PAs, not on one tree path)"
  ["multi-target", cites]
end

# Train mode (rules §8): the 5 compounds in both snapshots must get the same class in
# train-classes.yaml and the sealed class-labels.yaml. Records both fetches, prints the
# table, aborts on any mismatch (would be a within-release-37 drift -> flag A28).
def assert_overlap(train_labels, train_snap)
  abort "overlap check: sealed #{File.basename(SPLIT_OUT)} not found" unless File.exist?(SPLIT_OUT)
  split = YAML.safe_load(File.read(SPLIT_OUT))
  split_class = {}
  split["labels"].each { |l| split_class[norm(l["compound"])] = l["class"] }
  train_class = {}
  train_labels.each { |l| train_class[norm(canonical(l["compound"]))] = l["class"] }
  split_snap = JSON.parse(File.read(SPLIT_OUT.sub("class-labels.yaml", "class-labels-chembl37.json")))
  rows = OVERLAP.sort.map do |c|
    k = norm(c)
    [c, train_class[k], split_class[k]]
  end
  warn "overlap check (rules §8): the 5 compounds in both snapshots"
  warn "  train snapshot  fetched #{train_snap['fetched']} via #{train_snap['endpoints'].join(', ')}"
  warn "  sealed snapshot fetched #{split_snap['fetched']} via #{split_snap['endpoints'].join(', ')}"
  bad = []
  rows.each do |c, t, s|
    mark = (t == s && !t.nil?) ? "ok" : "MISMATCH"
    warn format("  %-24s train=%-22s sealed=%-22s %s", c, t.inspect, s.inspect, mark)
    bad << c unless t == s && !t.nil?
  end
  abort "overlap MISMATCH on #{bad.join(', ')} -> flag A28 (within-release-37 drift)" unless bad.empty?
  warn "  all 5 overlap classes match"
end

def build
  dir = ENV.fetch("MESH_DIR") { abort "set MESH_DIR to the folder holding #{MESH_FILES.join(', ')}" }
  src = YAML.safe_load(File.read(SOURCES))
  abort "ChEMBL snapshot sha mismatch" unless Digest::SHA256.file(SNAP).hexdigest == src["chembl"]["sha256"]
  src["mesh"].each do |m|
    abort "#{m['file']} sha mismatch" unless Digest::SHA256.file(File.join(dir, m["file"])).hexdigest == m["sha256"]
  end
  snap = JSON.parse(File.read(SNAP))
  desc = parse_desc(File.join(dir, "desc2026.xml"))
  pa_raw = parse_pa(File.join(dir, "pa2026.xml"))
  pa_index = pa_raw.transform_values { |subs| subs.map { |_, n| norm(n) } }
  labels = snap["compounds"].map do |entry|
    c = entry["compound"]
    cc = canonical(c)
    flags = []
    step, klass, cites = nil
    if CALLS.key?(cc)
      klass, basis = CALLS[cc]
      cites = chembl_cites(usable_records(entry), snap) + [basis]
      step = "S1-call"
    else
      klass, cites = class_from_chembl(entry, snap, flags)
      step = "S1" if klass
      if klass.nil?
        klass, cites = class_from_mesh(c, desc, pa_index, flags)
        step = "S2-skipped+S3" if klass
      end
      if klass.nil?
        klass = "unclassified"
        step = "S4"
        cites = []
        flags << (entry["molecules"].empty? ? "no ChEMBL molecule by exact name/alias" : "ChEMBL molecule(s) #{entry['molecules'].map { |m| m['parent'] }.join(', ')} without usable mechanism")
        flags << "no admissible MeSH PA"
      end
    end
    { "compound" => c, "class" => klass, "source_step" => step, "sources" => cites,
      "chembl_molecules" => entry["molecules"].map { |m| m["parent"] },
      "source_date" => snap["fetched"], "flags" => flags.sort,
      "proposed_by" => "A71 (class-labels.rb)", "approved_by" => nil, "approved_date" => nil }
  end
  assert_overlap(labels, snap) if TRAIN
  vocab = (labels.map { |l| l["class"] } + FIXED_VOCAB).uniq.sort_by(&:downcase)
  header = <<~H
    # GENERATED by class-labels.rb from class-labels-rules.md, class-labels-aliases.yaml,
    # #{File.basename(SNAP)} and pinned MeSH 2026 (#{File.basename(SOURCES)}). Do not edit.
    # approved_by / approved_date are set by Verification in the seal PR only.
  H
  header + { "vocabulary" => vocab, "labels" => labels }.to_yaml(line_width: -1)
end

case ARGV[0]
when "--fetch" then fetch
when "--pin" then pin(ARGV[1] || abort("usage: --pin DIR"))
when "--check"
  same = File.read(OUT) == build
  puts same ? "class-labels.yaml matches a rebuild" : "class-labels.yaml differs from a rebuild"
  exit(same ? 0 : 1)
else
  File.write(OUT, build)
  puts "wrote #{OUT}"
end
