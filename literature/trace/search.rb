#!/usr/bin/env ruby
# frozen_string_literal: true

# D118 trace search for the ITP blind-split cohorts (rule: README.md).
# Masked: never prints an abstract sentence containing an outcome word,
# never prints caption text or numbers.
#
#   ruby search.rb search            -> hits.tsv (masked screening table)
#   ruby search.rb locate PMID ...   -> location ids per paper
# Ruby 2.6 compatible (system Ruby).

require 'net/http'
require 'uri'
require 'json'
require 'rexml/document'

DIR = __dir__
FROM = '2021/01/01'
OUTCOME = /surviv|lifespan|life[- ]span|mortalit|death|longevity|kaplan|hazard|median/i
COHORT = /\bC20[2-9]\d\b|20[2-9]\d cohort|cohort (?:of |started (?:in )?)?20[2-9]\d|started in 20[2-9]\d|initiated in 20[2-9]\d/i

# Blind arms B01-B19 (research/metric/v0/split-proposal.md, "Blind part").
COMPOUNDS = {
  'B01' => ['dichloroacetate', 'DCA'],
  'B02' => ['TPPU'],
  'B03' => ['17alpha-estradiol', '17-alpha-estradiol', '17α-estradiol'],
  'B04/B05' => ['canagliflozin'],
  'B06' => ['sodium thiosulfate', 'thiosulfate'],
  'B07' => ['lactoferrin'],
  'B08' => ['captopril'],
  'B09' => ['trametinib'],
  'B10' => ['mianserin'],
  'B11' => ['cinnarizine'],
  'B12' => ['deprenyl', 'selegiline'],
  'B13' => ['sildenafil'],
  'B14' => ['alpelisib'],
  'B15' => ['alogliptin'],
  'B16' => ['BT2', 'branched-chain ketoacid dehydrogenase kinase inhibitor'],
  'B17' => ['glucosamine'],
  'B18' => ['taurine'],
  'B19' => ['urolithin A', 'urolithin']
}.freeze
ITP = '("Interventions Testing Program" OR UM-HET3 OR "genetically heterogeneous mice")'
SNYDER_DOI = '10.1007/s11357-025-01741-3'
A65_PMID = '35974129' # flagged by A65 (G150, 2026-10-21)

def get(url, params)
  uri = URI(url)
  uri.query = URI.encode_www_form(params)
  sleep 0.4
  res = Net::HTTP.get_response(uri)
  raise "HTTP #{res.code} #{uri}" unless res.is_a?(Net::HTTPSuccess)
  res.body
end

def or_terms(names)
  '(' + names.map { |n| %("#{n}") }.join(' OR ') + ')'
end

def queries
  q = [['Q0', 'snyder', "#{SNYDER_DOI}[doi]", %(DOI:"#{SNYDER_DOI}")],
       ['Q0', 'a65-flag', "#{A65_PMID}[pmid]", "EXT_ID:#{A65_PMID} AND SRC:MED"]]
  COMPOUNDS.each do |arm, names|
    q << ['Q1', arm, "#{or_terms(names)} AND #{ITP}", "#{or_terms(names)} AND #{ITP}"]
  end
  q << ['Q2', 'all', '"Interventions Testing Program"', '"Interventions Testing Program"']
  q
end

def pubmed(term, from)
  r = JSON.parse(get('https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi',
                     db: 'pubmed', retmode: 'json', retmax: 500, term: term,
                     datetype: 'pdat', mindate: from, maxdate: Time.now.strftime('%Y/%m/%d')))
  r['esearchresult']['idlist']
end

def epmc(query, from)
  q = "(#{query}) AND FIRST_PDATE:[#{from.tr('/', '-')} TO #{Time.now.strftime('%Y-%m-%d')}]"
  JSON.parse(get('https://www.ebi.ac.uk/europepmc/webservices/rest/search',
                 query: q, format: 'json', resultType: 'core', pageSize: 500))['resultList']['result']
end

def masked(abstract)
  sents = abstract.to_s.gsub(/<[^>]+>/, ' ').split(/(?<=[.;])\s+/)
  flag = sents.any? { |s| s =~ OUTCOME }
  cohort = sents.reject { |s| s =~ OUTCOME }.join(' ').scan(COHORT).uniq
  [flag, cohort]
end

def search
  rows = {}
  counts = []
  queries.each do |qid, arm, pm_term, ep_term|
    from = qid == 'Q2' ? '2022/01/01' : FROM
    pm = pubmed(pm_term, from)
    ep = epmc(ep_term, from)
    counts << [qid, arm, pm.size, ep.size]
    ep.each do |r|
      key = r['pmid'] || r['doi'] || r['id']
      flag, cohort = masked(r['abstractText'])
      row = rows[key] ||= { pmid: r['pmid'].to_s, doi: r['doi'].to_s, src: r['source'],
                            date: r['firstPublicationDate'], journal: r['journalTitle'] || r['bookOrReportDetails'].to_h['publisher'],
                            title: r['title'].to_s.gsub(/\s+/, ' '), arms: [], flag: flag, cohort: cohort }
      row[:arms] |= [arm]
    end
    pm.each { |id| rows[id] ||= { pmid: id, doi: '', src: 'MED-only', date: '', journal: '', title: '(PubMed only, not in Europe PMC result)', arms: [], flag: nil, cohort: [] }; rows[id][:arms] |= [arm] }
  end
  File.open(File.join(DIR, 'hits.tsv'), 'w') do |f|
    f.puts %w[pmid doi source date journal arms outcome_sentence_in_abstract cohort_tokens title].join("\t")
    rows.values.sort_by { |r| r[:date].to_s }.each do |r|
      flag = r[:flag].nil? ? 'unknown' : (r[:flag] ? 'yes' : 'no')
      f.puts [r[:pmid], r[:doi], r[:src], r[:date], r[:journal], r[:arms].join(','), flag, r[:cohort].join(';'), r[:title]].join("\t")
    end
  end
  counts.each { |c| puts c.join("\t") }
  puts "unique records: #{rows.size}"
end

def locate(pmids)
  pmids.each do |p|
    r = JSON.parse(get('https://www.ebi.ac.uk/europepmc/webservices/rest/search',
                       query: "EXT_ID:#{p} AND SRC:MED", format: 'json', resultType: 'lite'))['resultList']['result'][0]
    pmc = r && r['pmcid']
    unless pmc
      puts "#{p}\tabstract flag only, full text not open"
      next
    end
    doc = REXML::Document.new(get("https://www.ebi.ac.uk/europepmc/webservices/rest/#{pmc}/fullTextXML", {}))
    hits = []
    REXML::XPath.each(doc, '//sec') do |s|
      t = s.elements['title']&.text.to_s
      hits << "section: #{t.strip[0, 60]}" if t =~ OUTCOME
    end
    %w[fig table-wrap supplementary-material].each do |tag|
      REXML::XPath.each(doc, "//#{tag}") do |e|
        cap = REXML::XPath.match(e, './/caption//text() | .//title//text()').join(' ')
        next unless cap =~ OUTCOME
        label = e.elements['label']&.text || e.attributes['id'] || tag
        hits << "#{tag}: #{label.strip}"
      end
    end
    puts "#{p}\t#{pmc}\t#{hits.empty? ? 'no outcome heading or caption' : hits.uniq.join('; ')}"
  end
end

case ARGV.shift
when 'search' then search
when 'locate' then locate(ARGV)
else warn 'usage: ruby search.rb search | locate PMID ...'; exit 2
end
