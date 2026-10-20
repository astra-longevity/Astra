# Validates the L1 lifespan rows and computes test2_main (schema.md, ruling C3).
# Usage: ruby literature/lifespan/check.rb   (exit 1 on any error)
# Ruby standard library only (YAML/Psych).
require "yaml"
require "date"

DIR = File.expand_path(__dir__)
ROLES = %w[extends null shortens mixed].freeze
SE_SOURCES = [nil, "printed", "from_curve", "from_p_value"].freeze
SE_SCALES = [nil, "log_hr", "log_median_ratio"].freeze
AGREEMENT = [nil, "pass", "fail"].freeze

def load(path)
  YAML.safe_load(File.read(path), permitted_classes: [Date])
end

errors = []
labels_doc = load(File.join(DIR, "class-labels.yaml"))
vocab = labels_doc.fetch("vocabulary")
labels = {}
labels_doc.fetch("labels").each do |l|
  errors << "class-labels: #{l['compound']} class '#{l['class']}' not in vocabulary" unless vocab.include?(l["class"])
  errors << "class-labels: #{l['compound']} listed twice" if labels.key?(l["compound"])
  labels[l["compound"]] = l
end
ref_version = load(File.join(DIR, "reference-ranges.yaml")).fetch("version")

rows = Dir[File.join(DIR, "{itp,other}", "*.yaml")].sort
rows.each do |path|
  r = load(path)
  id = File.basename(path, ".yaml")
  err = ->(msg) { errors << "#{id}: #{msg}" }
  err.call("id does not match file name") unless r["id"] == id
  err.call("role '#{r['role']}' invalid") unless ROLES.include?(r["role"])
  err.call("sex must be F or M") unless %w[F M].include?(r["sex"])
  warn "#{id}: class label pending (no entry in class-labels.yaml)" unless labels.key?(r["compound"])
  err.call("class_label belongs in class-labels.yaml") if r.key?("class_label")
  err.call("test2_main is computed, not stored") if r.dig("flags")&.key?("test2_main")
  e = r.fetch("effect")
  err.call("se_source invalid") unless SE_SOURCES.include?(e["se_source"])
  err.call("se_scale invalid") unless SE_SCALES.include?(e["se_scale"])
  err.call("curve_agreement invalid") unless AGREEMENT.include?(e["curve_agreement"])
  err.call("se without se_scale") if e["se"] && e["se_scale"].nil?
  f = r.fetch("flags")
  err.call("control_in_ref_range must be true/false") unless [true, false].include?(f["control_in_ref_range"])
  err.call("reference must be reference-ranges.yaml@#{ref_version}") unless f["reference"] == "reference-ranges.yaml@#{ref_version}"
  if r["program"] == "other" && f["itp_or_replicated"] && f["replicated_by"].nil?
    err.call("non-ITP row marked replicated without replicated_by")
  end

  se_ok = !e["se"].nil? && (e["se_source"] == "printed" ||
    (e["se_source"] == "from_curve" && e["digitisations"].to_i >= 2 && e["curve_agreement"] == "pass"))
  main = f["control_in_ref_range"] == true && f["itp_or_replicated"] == true &&
         Array(r.dig("verification", "to_verify")).empty? &&
         !labels.dig(r["compound"], "approved_by").nil? && se_ok
  puts "#{id}: test2_main=#{main}"
end

if errors.empty?
  puts "OK: #{rows.size} row(s)"
else
  warn errors.join("\n")
  exit 1
end
