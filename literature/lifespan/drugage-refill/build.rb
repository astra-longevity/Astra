# Builds the R-Metric v0 retrospective refill list from DrugAge Build 5.
# Rule: verification/approvals/r-metric-v0-refill.md. Licence: LA-017.
# Usage: ruby build.rb <path/to/dataset.zip>   (Ruby stdlib + `unzip`)
#
# Reads ONLY these DrugAge columns: compound_name, species, strain, dosage,
# gender, ITP, pubmed_id. Outcome columns (lifespan, weight, significance) are
# never read or written.
require "csv"
require "digest"
require "open3"

ZIP_SHA256 = "d12f59717de60a207748f53bdbcdb484ed07e30d8608eb8e4a101e2a08d7fa80".freeze
DIR = File.expand_path(__dir__)
KEEP = %w[compound_name species strain dosage gender ITP pubmed_id].freeze
YEARS = (2015..2022).freeze

zip = ARGV.fetch(0) { abort "usage: ruby build.rb <dataset.zip>" }
sha = Digest::SHA256.file(zip).hexdigest
abort "zip sha256 #{sha} != pinned #{ZIP_SHA256}" unless sha == ZIP_SHA256

csv_text, status = Open3.capture2("unzip", "-p", zip, "drugage.csv")
abort "unzip failed" unless status.success?
table = CSV.parse(csv_text, headers: true)
missing = KEEP - table.headers
abort "missing columns: #{missing.join(', ')}" unless missing.empty?
rows = table.map { |r| KEEP.map { |c| [c, r[c].to_s.strip] }.to_h }

years = {}
CSV.foreach(File.join(DIR, "pubmed-years.tsv"), col_sep: "\t", headers: true) do |r|
  years[r["pmid"]] = r["pubmed_year"].to_i
end
excluded = CSV.read(File.join(DIR, "exclusions.csv"), headers: true).map { |r| r["pmid"] }.uniq

mouse = rows.select { |r| r["species"] == "Mus musculus" }
pmids = mouse.map { |r| r["pubmed_id"] }.uniq
unknown = pmids.reject { |p| years.key?(p) }
abort "no PubMed year for: #{unknown.join(', ')}" unless unknown.empty?

# One study = one PMID: an ITP flag on any arm excludes the whole study.
itp_pmids = mouse.select { |r| r["ITP"] == "Yes" }.map { |r| r["pubmed_id"] }.uniq
in_window = pmids.select { |p| YEARS.cover?(years[p]) }
kept = in_window - itp_pmids - excluded

out = mouse.select { |r| kept.include?(r["pubmed_id"]) }
           .sort_by { |r| [r["pubmed_id"].to_i, r["compound_name"], r["gender"], r["dosage"], r["strain"]] }
path = File.join(DIR, "refill-list.csv")
CSV.open(path, "w") do |f|
  f << %w[arm_id pmid year compound sex strain dose]
  out.each_with_index do |r, i|
    f << [format("RF%03d", i + 1), r["pubmed_id"], years[r["pubmed_id"]], r["compound_name"],
          r["gender"], r["strain"], r["dosage"]]
  end
end

puts "zip sha256 OK (Build 5)"
puts "mouse arms #{mouse.size}, studies #{pmids.size}"
puts "studies in PubMed year #{YEARS.min}-#{YEARS.max}: #{in_window.size}"
puts "  excluded ITP (DrugAge ITP flag): #{(in_window & itp_pmids).size}"
puts "  excluded exclusions.csv: #{((in_window - itp_pmids) & excluded).size}"
puts "kept studies #{kept.size}, arms #{out.size}"
puts "refill-list.csv sha256 #{Digest::SHA256.file(path).hexdigest}"
puts "build.rb sha256 #{Digest::SHA256.file(__FILE__).hexdigest}"
