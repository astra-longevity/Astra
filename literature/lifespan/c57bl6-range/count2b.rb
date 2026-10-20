# Count-only dry run for C57BL/6 run 2b (search-rule-run2b.md, "Counts").
# Usage: ruby literature/lifespan/c57bl6-range/count2b.rb   (Ruby stdlib, network)
# Reads query A (search-rule-run2.md) and B1-B5 (search-rule-run2b.md) verbatim.
# For each candidate: Europe PMC hitCount, PMIDs returned, new PMIDs = (A u Bk) - run 1.
# PMID lists stay in memory: nothing is written, no record, title or text is read.
require "net/http"
require "uri"
require "json"
require "rexml/document"
require "csv"
require "digest"

DIR = File.expand_path(__dir__)
RULE2 = File.join(DIR, "search-rule-run2.md")
RULE2B = File.join(DIR, "search-rule-run2b.md")
RULE2_SHA256 = "5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be".freeze # cleared, PR #102
RULE2B_SHA256 = "d6ef9139622aa59287776d18bf11872050871d832b14c33b268eee541be2eedb".freeze # committed 4799272 (longevity added, A65 hold)
EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"
EPMC = "https://www.ebi.ac.uk/europepmc/webservices/rest/search"
PAUSE = 0.4
MAX_NEW = 1500

{ RULE2 => RULE2_SHA256, RULE2B => RULE2B_SHA256 }.each do |f, want|
  got = Digest::SHA256.file(f).hexdigest
  abort "#{File.basename(f)} sha256 #{got} != #{want}" unless got == want
end

section = File.read(RULE2)[/^## Search \(fixed, run once\)(.*?)^## /m, 1]
blocks = section.to_s.scan(/```\s*\n(.*?)\n\s*```/m).map { |b| b[0].lines.map(&:strip).reject(&:empty?).join(" ") }
abort "expected 2 query blocks in run-2 rule, found #{blocks.size}" unless blocks.size == 2
query_a = blocks[0]

cands = File.read(RULE2B).scan(/^(B[1-5]): (.+)$/).map { |id, q| [id, q.strip] }
abort "expected B1-B5, found #{cands.map(&:first).join(',')}" unless cands.map(&:first) == %w[B1 B2 B3 B4 B5]

def epmc_pmids(query)
  pmids = []
  no_pmid = 0
  count = nil
  cursor = "*"
  loop do
    sleep PAUSE
    uri = URI(EPMC)
    uri.query = URI.encode_www_form("query" => query, "resultType" => "idlist", "format" => "json",
                                    "pageSize" => "1000", "cursorMark" => cursor)
    res = Net::HTTP.get_response(uri)
    abort "Europe PMC: HTTP #{res.code}" unless res.is_a?(Net::HTTPSuccess)
    page = JSON.parse(res.body)
    count ||= page["hitCount"].to_i
    results = page.dig("resultList", "result") || []
    results.each { |r| r["pmid"].to_s.empty? ? no_pmid += 1 : pmids << r["pmid"].to_s }
    nxt = page["nextCursorMark"]
    break if results.empty? || nxt.nil? || nxt == cursor
    cursor = nxt
  end
  abort "Europe PMC: #{count} hits, #{pmids.size + no_pmid} returned" if pmids.size + no_pmid < count
  [count, pmids.uniq, no_pmid]
end

sleep PAUSE
res = Net::HTTP.post_form(URI(EUTILS + "esearch.fcgi"), "db" => "pubmed", "term" => query_a, "retmax" => "10000",
                                                       "tool" => "astra-a71", "email" => "astra-longevity@users.noreply.github.com")
abort "PubMed: HTTP #{res.code}" unless res.is_a?(Net::HTTPSuccess)
doc = REXML::Document.new(res.body)
count_a = doc.elements["eSearchResult/Count"].text.to_i
pmids_a = []
doc.elements.each("eSearchResult/IdList/Id") { |e| pmids_a << e.text }
abort "retmax too small: #{count_a} hits, #{pmids_a.size} returned" if pmids_a.size < count_a

run1 = CSV.read(File.join(DIR, "hits.tsv"), col_sep: "\t", headers: true, quote_char: "\u0000").map { |r| r["pmid"] }

puts "run_date: #{Time.now.utc.strftime('%Y-%m-%d %H:%M UTC')}"
puts "rule2 sha256: #{RULE2_SHA256}"
puts "rule2b sha256: #{RULE2B_SHA256}"
puts "query_a_sha256: #{Digest::SHA256.hexdigest(query_a)}  A pubmed_count: #{count_a}"
puts "| Id | query sha256 | Europe PMC hitCount | PMIDs | no PMID | new PMIDs (A u Bk - run 1) | <= #{MAX_NEW} |"
puts "|---|---|---|---|---|---|---|"
results = cands.map do |id, q|
  count, pmids, no_pmid = epmc_pmids(q)
  new = ((pmids_a | pmids) - run1).size
  puts "| #{id} | `#{Digest::SHA256.hexdigest(q)[0, 16]}` | #{count} | #{pmids.size} | #{no_pmid} | #{new} | #{new <= MAX_NEW ? 'yes' : 'no'} |"
  [id, new]
end
ok = results.select { |_, n| n <= MAX_NEW }
chosen = ok.max_by { |id, n| [n, -id[1].to_i] }
puts chosen ? "choice rule: #{chosen[0]} (#{chosen[1]} new PMIDs)" : "choice rule: none <= #{MAX_NEW}; C57BL/6 stays pending"
puts "count2b.rb sha256: #{Digest::SHA256.file(__FILE__).hexdigest}"
