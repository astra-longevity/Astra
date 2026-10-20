# Runs the C57BL/6 control-range PubMed search exactly as fixed in search-rule.md.
# Usage: ruby literature/lifespan/c57bl6-range/search.rb   (Ruby stdlib, network)
# Writes hits.tsv and refill-authors.tsv (PubMed metadata only) and prints the
# lines for search-log.md. The query is read from search-rule.md itself, so it
# cannot drift from the committed rule.
require "net/http"
require "uri"
require "rexml/document"
require "csv"
require "digest"

DIR = File.expand_path(__dir__)
RULE = File.join(DIR, "search-rule.md")
REFILL = File.join(DIR, "..", "drugage-refill", "refill-list.csv")
EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"
PAUSE = 0.4 # NCBI: at most 3 requests/s without an API key

query = File.read(RULE)[/\*\*Query \(verbatim\):\*\*\s*```\s*\n(.*?)\n\s*```/m, 1]
abort "query block not found in search-rule.md" if query.nil?
query = query.lines.map(&:strip).reject(&:empty?).join(" ")

def get(path, params)
  sleep PAUSE
  uri = URI(EUTILS + path)
  res = Net::HTTP.post_form(uri, params.merge("tool" => "astra-a71", "email" => "astra-longevity@users.noreply.github.com"))
  abort "#{path}: HTTP #{res.code}" unless res.is_a?(Net::HTTPSuccess)
  res.body
end

# PubMed metadata for PMIDs: year, title, journal, authors, last-author affiliation.
def records(pmids)
  out = {}
  pmids.each_slice(200) do |batch|
    doc = REXML::Document.new(get("efetch.fcgi", "db" => "pubmed", "id" => batch.join(","), "retmode" => "xml"))
    doc.elements.each("PubmedArticleSet/PubmedArticle") do |a|
      pmid = a.elements["MedlineCitation/PMID"].text
      art = a.elements["MedlineCitation/Article"]
      year = art.elements["Journal/JournalIssue/PubDate/Year"]&.text ||
             art.elements["Journal/JournalIssue/PubDate/MedlineDate"]&.text.to_s[/\d{4}/]
      authors = []
      art.elements.each("AuthorList/Author") do |au|
        name = [au.elements["LastName"]&.text, au.elements["Initials"]&.text].compact.join(" ")
        name = au.elements["CollectiveName"]&.text.to_s if name.empty?
        aff = au.elements["AffiliationInfo/Affiliation"]&.text.to_s
        authors << [name, aff]
      end
      out[pmid] = {
        year: year.to_s,
        title: art.elements["ArticleTitle"]&.texts&.join.to_s,
        journal: art.elements["Journal/ISOAbbreviation"]&.text.to_s,
        authors: authors,
      }
    end
  end
  out
end

def clean(s)
  s.to_s.gsub(/[\t\r\n]+/, " ").strip
end

search = REXML::Document.new(get("esearch.fcgi", "db" => "pubmed", "term" => query, "retmax" => "10000"))
count = search.elements["eSearchResult/Count"].text.to_i
pmids = []
search.elements.each("eSearchResult/IdList/Id") { |e| pmids << e.text }
abort "retmax too small: #{count} hits, #{pmids.size} returned" if pmids.size < count
pmids = pmids.sort_by(&:to_i)

hits = records(pmids)
missing = pmids - hits.keys
abort "no record for: #{missing.join(', ')}" unless missing.empty?
CSV.open(File.join(DIR, "hits.tsv"), "w", col_sep: "\t") do |f|
  f << %w[pmid year journal last_author last_author_affiliation title]
  pmids.each do |p|
    r = hits[p]
    last = r[:authors].last || ["", ""]
    f << [p, r[:year], clean(r[:journal]), clean(last[0]), clean(last[1]), clean(r[:title])]
  end
end

refill_pmids = CSV.read(REFILL, headers: true).map { |r| r["pmid"] }.uniq.sort_by(&:to_i)
refill = records(refill_pmids)
CSV.open(File.join(DIR, "refill-authors.tsv"), "w", col_sep: "\t") do |f|
  f << %w[pmid last_author last_author_affiliation all_authors]
  refill_pmids.each do |p|
    r = refill.fetch(p) { abort "no record for refill PMID #{p}" }
    last = r[:authors].last || ["", ""]
    f << [p, clean(last[0]), clean(last[1]), r[:authors].map { |a| clean(a[0]) }.join("; ")]
  end
end

puts "run_date: #{Time.now.utc.strftime('%Y-%m-%d %H:%M UTC')}"
puts "query_sha256: #{Digest::SHA256.hexdigest(query)}"
puts "pubmed_count: #{count}"
puts "hits.tsv rows: #{pmids.size}, sha256 #{Digest::SHA256.file(File.join(DIR, 'hits.tsv')).hexdigest}"
puts "refill-authors.tsv rows: #{refill_pmids.size}, sha256 #{Digest::SHA256.file(File.join(DIR, 'refill-authors.tsv')).hexdigest}"
puts "search.rb sha256: #{Digest::SHA256.file(__FILE__).hexdigest}"
