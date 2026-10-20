# Stage-1 helper for run 2b: fetches title, abstract, publication types and MeSH
# headings of every PMID in hits-run2b.tsv from PubMed efetch, and writes them
# to a local JSONL (not committed: PubMed abstracts are publisher text).
# Stage-1 decisions are written to screening.csv; this file only feeds them.
#
#   ruby abstracts2b.rb OUT.jsonl
require "csv"
require "json"
require "net/http"
require "uri"
require "rexml/document"

out = ARGV[0] || abort("usage: ruby abstracts2b.rb OUT.jsonl")
pmids = CSV.read(File.join(__dir__, "hits-run2b.tsv"), col_sep: "\t", headers: true, quote_char: "\x00")
           .map { |r| r["pmid"] }
base = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi"
File.open(out, "w") do |f|
  pmids.each_slice(200) do |batch|
    res = Net::HTTP.post_form(URI(base), "db" => "pubmed", "id" => batch.join(","), "retmode" => "xml")
    raise "efetch #{res.code}" unless res.is_a?(Net::HTTPSuccess)
    doc = REXML::Document.new(res.body)
    doc.elements.each("PubmedArticleSet/PubmedArticle") do |a|
      txt = ->(path) { a.get_elements(path).map { |e| e.texts.join + e.get_elements(".//*").map { |c| c.texts.join }.join }.join(" ").strip }
      rec = {
        "pmid" => a.elements["MedlineCitation/PMID"].text,
        "title" => txt.call("MedlineCitation/Article/ArticleTitle"),
        "abstract" => a.get_elements("MedlineCitation/Article/Abstract/AbstractText").map { |e| REXML::XPath.match(e, ".//text()").join }.join(" "),
        "pubtypes" => a.get_elements("MedlineCitation/Article/PublicationTypeList/PublicationType").map(&:text),
        "mesh" => a.get_elements("MedlineCitation/MeshHeadingList/MeshHeading/DescriptorName").map(&:text)
      }
      rec["title"] = REXML::XPath.match(a.elements["MedlineCitation/Article/ArticleTitle"], ".//text()").join
      f.puts JSON.generate(rec)
    end
    sleep 0.5
  end
end
puts "wrote #{out}: #{File.foreach(out).count} records for #{pmids.size} PMIDs"
