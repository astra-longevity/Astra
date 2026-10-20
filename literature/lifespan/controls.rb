# Generates controls.md (T1-A2-2 negative controls, T5-A2-3 positive controls)
# from the L1 rows in itp/. Nothing is extracted here: every number comes from a
# row file, and every entry cites its row id.
# Usage: ruby literature/lifespan/controls.rb   (Ruby stdlib; writes controls.md)
# Exit 1 if controls.md on disk differs from what the rows give (--check).
require "yaml"
require "date"
require "digest"
require "csv"

DIR = File.expand_path(__dir__)
OUT = File.join(DIR, "controls.md")
# Named in the mission brief (A2, G152) and the specs; listed even when absent from L1.
NAMED_NEGATIVE = ["metformin", "resveratrol", "green tea extract", "curcumin", "simvastatin"].freeze
NAMED_POSITIVE = ["rapamycin", "acarbose", "17-alpha-estradiol", "ndga", "canagliflozin"].freeze

# Version of the lists (A50 approval of PR #99, condition 4). Later versions only by
# regeneration: bump VERSION, keep the older roles file, the diff goes in controls.md.
VERSION = 2
ROLES_V1 = File.join(DIR, "controls-roles-v1.tsv")
REFILL = File.join(DIR, "drugage-refill", "refill-list.csv")
REPO = File.expand_path("../..", DIR)
# Machine-readable sealed split (D115). Not on main yet; checked as soon as it exists.
# CONTROLS_SPLIT_GLOB overrides the location (used only by the firing test).
SPLIT_GLOB = ENV.fetch("CONTROLS_SPLIT_GLOB", File.join(REPO, "research", "metric", "v0", "split*.{csv,tsv,yaml,yml}"))
# Compounds with blind arms in the split (research/metric/v0/split-proposal.md). Their L1
# rows stay in the table (they train) but are never controls while any arm is blind
# (A50 ruling 2026-10-20, G155).
BLIND_COMPOUNDS = { "canagliflozin" => "blind arms B04/B05 (A50 ruling 2026-10-20)" }.freeze

# Compound key: lower case, Greek letters spelled out, every non-alphanumeric removed
# ("17α-estradiol" and "17-alpha-estradiol" both give "17alphaestradiol").
def norm(name)
  name.to_s.downcase.gsub("α", "alpha").gsub("β", "beta").gsub(/[^a-z0-9]/, "")
end

# A combination ("Rapamycin + 17α-estradiol", "metformin and rapamycin") counts as the
# whole name and as each component, on both sides of the comparison (A2, review of #109).
def parts(name)
  comps = name.to_s.downcase.split(/\s*\+\s*|\s+and\s+|\s*\/\s*/)
  ([norm(name)] + comps.map { |c| norm(c) }).reject(&:empty?).uniq
end

rows = Dir[File.join(DIR, "itp", "*.yaml")].sort.map do |p|
  YAML.safe_load(File.read(p), permitted_classes: [Date])
end
blind_rows = rows.select { |r| BLIND_COMPOUNDS.key?(r["compound"]) }
controls = rows.select { |r| %w[null extends].include?(r["role"]) } - blind_rows

# Condition 1: no control arm may sit in the sealed split or in the refill list.
problems = []
refill = CSV.read(REFILL, headers: true)
refill_ids = refill.map { |r| r["arm_id"] }
refill_pmids = refill.map { |r| r["pmid"] }
split_files = Dir[SPLIT_GLOB].sort
split_text = split_files.map { |f| File.read(f) }.join("\n")
controls.each do |r|
  problems << "#{r['id']}: arm id is in refill-list.csv" if refill_ids.include?(r["id"])
  pmid = r.dig("publication", "pmid").to_s
  problems << "#{r['id']}: publication PMID #{pmid} is in refill-list.csv" if !pmid.empty? && refill_pmids.include?(pmid)
  problems << "#{r['id']}: arm id appears in sealed split #{split_files.join(', ')}" if split_text.include?(r["id"])
end
# Compound-level test (A50 canagliflozin ruling, A65 item 1): the split's blind arms are
# B01-B19, not L1 ids, so their compound names are read from a `compound` column (CSV/TSV)
# or `compound` keys (YAML). A control whose compound equals a blind compound fails.
SEEN_TAG = "compound seen in training"
# One record per blind split arm: compound parts, cohort year, tag (A50 option 4, G155).
# The tag is read from the `flag` column only (A65 #142 change): the single encoding A50
# accepted. A tag written in `stratum` does not count.
blind_arm = lambda do |h|
  { parts: parts(h["compound"]), cohort: h["cohort"].to_s[/\d{4}/],
    tagged: h["flag"].to_s.downcase.include?(SEEN_TAG) }
end
split_arms = split_files.flat_map do |f|
  if f.end_with?(".yaml", ".yml")
    doc = YAML.safe_load(File.read(f), permitted_classes: [Date])
    walk = lambda do |x|
      case x
      when Hash then (x.key?("compound") ? [blind_arm.call(x)] : []) + x.values.flat_map { |v| walk.call(v) }
      when Array then x.flat_map { |v| walk.call(v) }
      else []
      end
    end
    walk.call(doc)
  else
    table = CSV.read(f, headers: true, col_sep: f.end_with?(".tsv") ? "\t" : ",")
    problems << "#{f}: no `compound` column, cannot run the compound-level check" unless table.headers.include?("compound")
    # Only blind arms count (A2, 2026-10-20): split.csv also lists the retrospective
    # refill arms (part=retro), whose compounds may train and be controls.
    table = table.select { |r| r["part"].to_s.strip.downcase == "blind" } if table.headers.include?("part")
    table.reject { |r| r["compound"].nil? }.map { |r| blind_arm.call(r.to_h) }
  end
end
# A50 option 4 (G155, supersedes the canagliflozin point 2): a control sharing a compound
# AND the cohort with a blind arm fails; compound-only overlap passes only if that blind
# arm carries the "compound seen in training" tag in the split file.
controls.each do |r|
  cparts = parts(r["compound"])
  split_arms.each do |a|
    hit = cparts & a[:parts]
    next if hit.empty?
    if a[:cohort] && a[:cohort] == r["cohort_year"].to_s
      problems << "#{r['id']}: compound '#{r['compound']}' (#{hit.join(', ')}) and cohort #{a[:cohort]} match a blind split arm"
    elsif !a[:tagged]
      problems << "#{r['id']}: compound '#{r['compound']}' (#{hit.join(', ')}) has a blind split arm not tagged '#{SEEN_TAG}'"
    end
  end
end
problems.uniq!
split_note = if split_files.empty?
               "no machine-readable sealed split file exists yet (`research/metric/v0/split*`); " \
                 "the check runs automatically once it does"
             else
               "checked against #{split_files.map { |f| "`#{f.sub("#{REPO}/", '')}`" }.join(', ')}"
             end

# Condition 4: role diff against the v1 snapshot (written once, when absent).
current_roles = controls.map { |r| [r["id"], r["role"]] }.to_h
unless File.exist?(ROLES_V1) || ARGV.include?("--check")
  File.write(ROLES_V1, "id\trole\n" + current_roles.sort.map { |i, ro| "#{i}\t#{ro}" }.join("\n") + "\n")
end
v1_roles = File.exist?(ROLES_V1) ? CSV.read(ROLES_V1, col_sep: "\t", headers: true).map { |r| [r["id"], r["role"]] }.to_h : {}
diff = []
(current_roles.keys - v1_roles.keys).sort.each { |i| diff << "- added: `#{i}` (#{current_roles[i]})" }
(v1_roles.keys - current_roles.keys).sort.each { |i| diff << "- removed: `#{i}` (was #{v1_roles[i]})" }
(current_roles.keys & v1_roles.keys).sort.each do |i|
  diff << "- role changed: `#{i}` #{v1_roles[i]} -> #{current_roles[i]}" if current_roles[i] != v1_roles[i]
end

def fmt(x)
  x.nil? ? "n/p" : format("%+.1f", x)
end

def dose(r)
  d = r["dose"]
  "#{d['value']} #{d['unit']}"
end

def logrank(r)
  pv = Array(r["p_values"]).find { |p| p["test"].to_s.start_with?("log-rank") }
  pv ? pv["value"].to_s : "n/p"
end

def interval(vals)
  v = vals.compact
  return "no printed median effect" if v.empty?
  lo, hi = v.minmax
  "[#{fmt(lo)}, #{fmt(hi)}] % (#{v.size} arm#{v.size == 1 ? '' : 's'} with a median effect)"
end

def table(rows)
  lines = ["| Row | Cohort | Dose | Start (d) | Median % | p90 % | Log-rank p | Control in range |",
           "|---|---|---|---|---|---|---|---|"]
  rows.sort_by { |r| [r["cohort_year"], r["dose"]["value"].to_s.to_f, r["start_age_days"]] }.each do |r|
    lines << "| `#{r['id']}` | C#{r['cohort_year']} | #{dose(r)} | #{r['start_age_days']} | " \
             "#{fmt(r.dig('effect', 'median_pct'))} | #{fmt(r.dig('effect', 'p90_pct'))} | #{logrank(r)} | " \
             "#{r.dig('flags', 'control_in_ref_range')} |"
  end
  lines.join("\n")
end

def section(title, rows, role, named)
  out = ["## #{title}", ""]
  by = rows.select { |r| r["role"] == role }.group_by { |r| r["compound"] }
  missing = named.reject { |c| by.key?(c) }.map { |c| BLIND_COMPOUNDS.key?(c) ? "#{c} (in L1, excluded while blind)" : c }
  by.keys.sort_by { |c| [named.include?(c) ? 0 : 1, c] }.each do |c|
    tag = named.include?(c) ? " (named control)" : ""
    out << "### #{c}#{tag}" << ""
    %w[F M].each do |sex|
      arms = by[c].select { |r| r["sex"] == sex }
      next if arms.empty?
      out << "**#{sex}**: interval #{interval(arms.map { |r| r.dig('effect', 'median_pct') })}" << ""
      out << table(arms) << ""
    end
  end
  out << "Named controls with no #{role} arm in L1 yet: #{missing.empty? ? 'none' : missing.join(', ')}." << ""
  out.join("\n")
end

other = rows.reject { |r| %w[null extends].include?(r["role"]) }
md = <<~MD
  # ITP control arms (L1), v#{VERSION}

  Generated by `controls.rb` from the rows in `itp/` (#{rows.size} rows). Do not edit by hand:
  change the rows, then re-run the script. Nothing here is extracted from a paper directly;
  each entry cites its row, where the source table and DOI are recorded.

  **Approval (A50, PR #99) and its conditions:**
  - **Version.** v1 = the lists approved in PR #99 (draft sha256 `e3eed40e…be59`; same tables,
    this header added). Roles of v1 are frozen in `controls-roles-v1.tsv`. Later versions
    (canagliflozin, C2015 onward) only by regeneration, with the role diff below. This is v#{VERSION}.
  - **Kept out of the test sets.** `controls.rb --check` fails if a control arm id (or its
    publication PMID, when recorded) is in `drugage-refill/refill-list.csv` or in the sealed
    split, or if a control's compound has any arm in the sealed split (compound-level test,
    A50 ruling 2026-10-20). A combination counts as each of its components, on both sides
    (split arm "rapamycin + 17α-estradiol" blocks every rapamycin and 17α-estradiol control).
    Split: #{split_note}.
  - **Interval = descriptive only.** It is not a confidence interval and not a pass/fail bar.
    Pass/fail tests on positive controls are defined by A28's sealed test-1 design, not by
    this file.
  - **Blind spot: shortening side empty.** No control here shortens lifespan, so the model's
    ability to detect harm is unchecked. The test scorecard must state this until a shortening
    control exists (a later rule-first, non-ITP, non-refill source).

  - **Negative controls** (T1-A2-2): arms whose row has `role: null` (log-rank not significant).
  - **Positive controls** (T5-A2-3, extending side): arms whose row has `role: extends`.
    No ITP arm shortens lifespan, so the shortening side (progeroid lines, high-fat diet,
    irradiation) needs non-ITP rows and is empty here.
  - **Interval** = observed minimum and maximum of the median effect (`effect.median_pct`,
    treated vs control, %) across that compound's arms for one sex, all doses, start ages and
    cohorts. It is **not a confidence interval**: no ITP paper prints an SE or hazard ratio.
  - "n/p" = not printed in the paper (open in the row's `to_verify` until the MPD files arrive).
  - Role follows the pooled log-rank test as recorded in the row. Single-site effects and
    replication notes are in each row's `notes`; this file makes no judgment of its own.
  - Approval of these lists: Verification (A50).

  #{section('Negative controls (role: null)', controls, 'null', NAMED_NEGATIVE)}
  #{section('Positive controls (role: extends)', controls, 'extends', NAMED_POSITIVE)}
  ## Excluded while blind

  Compounds with an arm in the blind part of the split are never controls (A50 ruling,
  2026-10-20). Their rows stay in L1 and train as usual.

  #{blind_rows.empty? ? 'None.' : blind_rows.map { |r| "- `#{r['id']}` (#{r['role']}): #{BLIND_COMPOUNDS[r['compound']]}" }.join("\n")}

  ## Other roles

  #{other.empty? ? 'None (no row has role shortens or mixed).' : other.map { |r| "- `#{r['id']}`: #{r['role']}" }.join("\n")}

  ## Role changes since v1

  #{diff.empty? ? 'None.' : diff.join("\n")}
MD

unless problems.empty?
  warn problems.join("\n")
  warn "control arms must stay out of the refill list and the sealed split (A50, PR #99)"
  exit 1
end

if ARGV.include?("--check")
  if File.exist?(OUT) && File.read(OUT) == md
    puts "controls.md up to date"
  else
    warn "controls.md differs from the rows: run ruby literature/lifespan/controls.rb"
    exit 1
  end
else
  File.write(OUT, md)
  puts "controls.md written from #{rows.size} rows, sha256 #{Digest::SHA256.file(OUT).hexdigest}"
end
