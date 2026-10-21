# Open-copy lookup (search-rule-run2.md, "Open-copy lookup", A50 condition 2)
# for every run-2b stage-1 pass and every run-1 X_NO_FULLTEXT line.
# Fixed order, first open copy wins:
#   1. PMC / Europe PMC (NCBI idconv, Europe PMC fullTextUrlList, author manuscripts)
#   2. Unpaywall (best_oa_location, then every oa_locations entry)
#   3. Preprint (Europe PMC SRC:PPR title search; bioRxiv/medRxiv). A preprint is only
#      a candidate: stage 2 accepts it only if its methods describe the same cohort.
# Metadata only: no full text is downloaded or read here.
#
#   ruby oa2b.rb > oa-lookup-run2b.tsv
require "csv"
require "json"
require "net/http"
require "uri"

EMAIL = "astra-longevity@users.noreply.github.com"

def get_json(url)
  3.times do |i|
    res = Net::HTTP.get_response(URI(url))
    return JSON.parse(res.body) if res.is_a?(Net::HTTPSuccess)
    return nil if res.code == "404"
    sleep 2 * (i + 1)
  end
  nil
rescue StandardError
  nil
ensure
  sleep 0.4
end

def esc(s)
  URI.encode_www_form_component(s)
end

rows = CSV.read(File.join(__dir__, "screening.csv"), headers: true)
todo = rows.select { |r| (r["run"] == "2b" && r["decision"] == "pass") || (r["run"] == "1" && r["reason_code"] == "X_NO_FULLTEXT") }
           .map { |r| [r["pmid"], r["run"]] }.uniq

puts %w[pmid run doi route url].join("\t")
todo.each do |pmid, run|
  route = url = nil
  # 1. PMC / Europe PMC
  ep = get_json("https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=EXT_ID:#{pmid}%20AND%20SRC:MED&resultType=core&format=json")
  rec = ep && ep.dig("resultList", "result", 0)
  doi = rec && rec["doi"]
  title = rec && rec["title"]
  pmcid = rec && rec["pmcid"]
  if pmcid.nil?
    ic = get_json("https://www.ncbi.nlm.nih.gov/pmc/utils/idconv/v1.0/?ids=#{pmid}&format=json&tool=astra&email=#{esc(EMAIL)}")
    pmcid = ic && ic.dig("records", 0, "pmcid")
    doi ||= ic && ic.dig("records", 0, "doi")
  end
  if pmcid
    route, url = "pmc", "https://pmc.ncbi.nlm.nih.gov/articles/#{pmcid}/"
  elsif rec
    free = (rec.dig("fullTextUrlList", "fullTextUrl") || []).find { |u| %w[Free Open\ access].include?(u["availability"]) }
    route, url = "europepmc", free["url"] if free
  end
  # 2. Unpaywall
  if route.nil? && doi
    up = get_json("https://api.unpaywall.org/v2/#{esc(doi)}?email=#{esc(EMAIL)}")
    if up
      loc = up["best_oa_location"] || (up["oa_locations"] || []).first
      u = loc && (loc["url_for_pdf"] || loc["url"])
      route, url = "unpaywall", u if u
    end
  end
  # 3. Preprint candidate (title search)
  if route.nil? && title
    q = esc(%(TITLE:"#{title.sub(/\.\z/, '').gsub('"', '')}" AND SRC:PPR))
    pp = get_json("https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=#{q}&format=json")
    hit = pp && pp.dig("resultList", "result", 0)
    route, url = "preprint_candidate", "https://europepmc.org/article/PPR/#{hit['id']}" if hit
  end
  route ||= "closed"
  puts [pmid, run, doi, route, url].map(&:to_s).join("\t")
  $stdout.flush
end
