# Runs the C57BL/6 control-range search, run 2, exactly as fixed in search-rule-run2.md.
# Usage: ruby literature/lifespan/c57bl6-range/search2.rb   (Ruby stdlib, network)
# Reads queries A (PubMed) and B (Europe PMC full text) verbatim from the rule file,
# takes the union of PMIDs, drops run-1 PMIDs (hits.tsv), writes hits-run2.tsv with
# PubMed metadata only, and prints the lines for search-log.md.
# Stops before writing anything if there are more than 1500 new PMIDs (rule).
require "net/http"
require "uri"
require "json"
require "rexml/document"
require "csv"
require "digest"

DIR = File.expand_path(__dir__)
RULE = File.join(DIR, "search-rule-run2.md")
RULE_SHA256 = "5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be".freeze # cleared by A65, PR #102
EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"
EPMC = "https://www.ebi.ac.uk/europepmc/webservices/rest/search"
PAUSE = 0.4
MAX_NEW = 1500

sha = Digest::SHA256.file(RULE).hexdigest
abort "search-rule-run2.md sha256 #{sha} != cleared #{RULE_SHA256}" unless sha == RULE_SHA256

section = File.read(RULE)[/^## Search \(fixed, run once\)(.*?)^## /m, 1]
blocks = section.to_s.scan(/```\s*\n(.*?)\n\s*```/m).map { |b| b[0].lines.map(&:strip).reject(&:empty?).join(" ") }
abort "expected 2 query blocks, found #{blocks.size}" unless blocks.size == 2
query_a, query_b = blocks

def post(url, params)
  sleep PAUSE
  res = Net::HTTP.post_form(URI(url), params)
  abort "#{url}: HTTP #{res.code}" unless res.is_a?(Net::HTTPSuccess)
  res.body
end

def eutils(path, params)
  post(EUTILS + path, params.merge("tool" => "astra-a71", "email" => "astra-longevity@users.noreply.github.com"))
end

# A. PubMed
search = REXML::Document.new(eutils("esearch.fcgi", "db" => "pubmed", "term" => query_a, "retmax" => "10000"))
count_a = search.elements["eSearchResult/Count"].text.to_i
pmids_a = []
search.elements.each("eSearchResult/IdList/Id") { |e| pmids_a << e.text }
abort "retmax too small: #{count_a} hits, #{pmids_a.size} returned" if pmids_a.size < count_a

# B. Europe PMC full text, all pages via cursorMark
pmids_b = []
no_pmid_b = 0
count_b = nil
cursor = "*"
loop do
  sleep PAUSE
  uri = URI(EPMC)
  uri.query = URI.encode_www_form("query" => query_b, "resultType" => "lite", "format" => "json",
                                  "pageSize" => "1000", "cursorMark" => cursor)
  res = Net::HTTP.get_response(uri)
  abort "Europe PMC: HTTP #{res.code}" unless res.is_a?(Net::HTTPSuccess)
  page = JSON.parse(res.body)
  count_b ||= page["hitCount"].to_i
  results = page.dig("resultList", "result") || []
  results.each { |r| r["pmid"].to_s.empty? ? no_pmid_b += 1 : pmids_b << r["pmid"].to_s }
  nxt = page["nextCursorMark"]
  break if results.empty? || nxt.nil? || nxt == cursor
  cursor = nxt
end
abort "Europe PMC: #{count_b} hits, #{pmids_b.size + no_pmid_b} returned" if pmids_b.size + no_pmid_b < count_b

pmids_b.uniq!
run1 = CSV.read(File.join(DIR, "hits.tsv"), col_sep: "\t", headers: true, quote_char: "\u0000").map { |r| r["pmid"] }
union = (pmids_a | pmids_b)
new = (union - run1).sort_by(&:to_i)

puts "run_date: #{Time.now.utc.strftime('%Y-%m-%d %H:%M UTC')}"
puts "rule sha256: #{sha}"
puts "query_a_sha256: #{Digest::SHA256.hexdigest(query_a)}"
puts "query_b_sha256: #{Digest::SHA256.hexdigest(query_b)}"
puts "A pubmed_count: #{count_a}"
puts "B europepmc_count: #{count_b}, with PMID #{pmids_b.size} (unique), without PMID #{no_pmid_b}"
puts "union: #{union.size}; in run 1: #{(union & run1).size}; new: #{new.size} (A only #{(new - pmids_b).size}, B only #{(new - pmids_a).size}, A+B #{(new & pmids_a & pmids_b).size})"
if new.size > MAX_NEW
  puts "STOP: #{new.size} new PMIDs > #{MAX_NEW}. Nothing written; report to A2/A50 (rule, Size)."
  exit 3
end

def records(pmids)
  out = {}
  pmids.each_slice(200) do |batch|
    doc = REXML::Document.new(eutils("efetch.fcgi", "db" => "pubmed", "id" => batch.join(","), "retmode" => "xml"))
    doc.elements.each("PubmedArticleSet/PubmedArticle") do |a|
      pmid = a.elements["MedlineCitation/PMID"].text
      art = a.elements["MedlineCitation/Article"]
      year = art.elements["Journal/JournalIssue/PubDate/Year"]&.text ||
             art.elements["Journal/JournalIssue/PubDate/MedlineDate"]&.text.to_s[/\d{4}/]
      authors = []
      art.elements.each("AuthorList/Author") do |au|
        name = [au.elements["LastName"]&.text, au.elements["Initials"]&.text].compact.join(" ")
        name = au.elements["CollectiveName"]&.text.to_s if name.empty?
        authors << [name, au.elements["AffiliationInfo/Affiliation"]&.text.to_s]
      end
      out[pmid] = { year: year.to_s, title: art.elements["ArticleTitle"]&.texts&.join.to_s,
                    journal: art.elements["Journal/ISOAbbreviation"]&.text.to_s, authors: authors }
    end
  end
  out
end

def clean(s)
  s.to_s.gsub(/[\t\r\n]+/, " ").strip
end

meta = records(new)
missing = new - meta.keys
abort "no PubMed record for: #{missing.join(', ')}" unless missing.empty?
out = File.join(DIR, "hits-run2.tsv")
CSV.open(out, "w", col_sep: "\t") do |f|
  f << %w[pmid year journal last_author last_author_affiliation title source]
  new.each do |p|
    r = meta[p]
    last = r[:authors].last || ["", ""]
    src = [pmids_a.include?(p) ? "A" : nil, pmids_b.include?(p) ? "B" : nil].compact.join("+")
    f << [p, r[:year], clean(r[:journal]), clean(last[0]), clean(last[1]), clean(r[:title]), src]
  end
end
puts "hits-run2.tsv rows: #{new.size}, sha256 #{Digest::SHA256.file(out).hexdigest}"
puts "search2.rb sha256: #{Digest::SHA256.file(__FILE__).hexdigest}"
