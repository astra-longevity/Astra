#!/usr/bin/env ruby
# frozen_string_literal: true

# Independent-labs check for L3 evidence grades (README, "Independent labs").
# Two studies are from independent labs when their PubMed author lists share
# no author. An author is matched on normalised last name + first initial.
#
# Usage: ruby independent-labs.rb PMID PMID [PMID ...]
# Prints every pair with the shared authors. Exit 0 if all pairs are
# independent, 1 if any pair shares an author, 2 on a fetch or input error.

require 'net/http'
require 'uri'
require 'rexml/document'

EFETCH = 'https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi'

def fail_input(msg)
  warn msg
  exit 2
end

def normalise(text)
  text.unicode_normalize(:nfkd).gsub(/\p{Mn}/, '').downcase.gsub(/[^a-z]/, '')
end

def authors(pmid)
  uri = URI(EFETCH)
  uri.query = URI.encode_www_form(db: 'pubmed', id: pmid, retmode: 'xml')
  body = Net::HTTP.get_response(uri)
  fail_input("fetch failed for #{pmid}: HTTP #{body.code}") unless body.is_a?(Net::HTTPSuccess)
  doc = REXML::Document.new(body.body)
  list = REXML::XPath.match(doc, '//AuthorList/Author').map do |a|
    last = a.elements['LastName']&.text
    first = a.elements['ForeName']&.text || a.elements['Initials']&.text
    next if last.nil? || first.nil?
    "#{normalise(last)} #{normalise(first)[0]}"
  end.compact
  fail_input("no authors parsed for #{pmid}") if list.empty?
  list.uniq
end

pmids = ARGV
if pmids.size < 2 || pmids.any? { |p| p !~ /\A\d+\z/ }
  warn 'usage: ruby independent-labs.rb PMID PMID [PMID ...]'
  exit 2
end

lists = pmids.to_h do |p|
  sleep 0.4
  [p, authors(p)]
end

shared_any = false
pmids.combination(2) do |a, b|
  shared = lists[a] & lists[b]
  shared_any ||= shared.any?
  verdict = shared.empty? ? 'independent' : "same lab (#{shared.size} shared: #{shared.join(', ')})"
  puts "#{a} vs #{b}: #{verdict}"
end
exit(shared_any ? 1 : 0)
