#!/usr/bin/env ruby
# Scripted join of class labels into the R-Metric v0 split, and re-derivation
# of the a/b/c stratum. Requested by A28 (#139, G144 2026-10-21).
#
# What it does, in one pass, with no hand edits:
#   1. class_label  <- literature/lifespan/class-labels.yaml, keyed on `compound`
#      (the yaml labels all 41 split compounds; the match is 1:1 by name).
#   2. stratum      <- re-derived a/b/c ONLY (A28 rule, G144 2026-10-21):
#        c  : combination or sex-transfer arm (detected from `flag`);
#        a  : else, iff class_label is a real class (not `unclassified`,
#             not `multi-target`) AND that class has >=1 TRAIN arm;
#        b  : else (every unclassified / multi-target arm, and any real
#             class with no train example).
#      The "compound seen in training" tag stays in `flag`, never in `stratum`
#      (A65 #144): a scorecard grouping on stratum must not mix class-novelty
#      with leakage.
#
# TRAIN classes: the train set is the 132 finished ITP arms in
# literature/lifespan/itp/, NOT rows of this file. The set of classes present
# in train therefore needs each train compound's class, owned by L1:
# literature/lifespan/train-classes.yaml (A71 #185). Only single-compound
# train entries with a real class count (rule 2a, see train_classes below).
# TRAIN_CLASSES_FILE = nil reproduces the old provisional run (#183).
#
#   ruby join-class-labels.rb            # rewrites split.csv in place, prints counts
#   ruby join-class-labels.rb --check    # prints counts only, writes nothing

require "yaml"
require "csv"
require "set"

DIR          = __dir__
SPLIT        = File.join(DIR, "split.csv")
LABELS       = File.expand_path("../../../literature/lifespan/class-labels.yaml", DIR)
# L1 train-compound classes (A71 #185, checked by A65 #186). nil => provisional.
TRAIN_CLASSES_FILE = File.expand_path("../../../literature/lifespan/train-classes.yaml", DIR)

NON_CLASS = ["unclassified", "multi-target"].freeze

def load_labels
  doc = YAML.safe_load(File.read(LABELS))
  doc.fetch("labels").each_with_object({}) do |l, h|
    h[l.fetch("compound")] = l.fetch("class")
  end
end

# Rule 2a (A28, G171 2026-10-21): a class is present in train iff it is the
# label of a SINGLE-compound train entry. Combination entries ("x + y") and
# unclassified / multi-target entries contribute nothing; a class seen only
# inside a train combination is NOT present (no component decomposition).
# Class strings match exactly; no hand merging of near-identical names.
def train_classes(_label_of)
  return nil if TRAIN_CLASSES_FILE.nil? # provisional: every real class counts
  YAML.safe_load(File.read(TRAIN_CLASSES_FILE))
      .fetch("labels")
      .reject { |l| l.fetch("compound").include?(" + ") }
      .map { |l| l.fetch("class") }
      .select { |c| real_class?(c) }
      .to_set
end

def real_class?(cls)
  cls && !NON_CLASS.include?(cls)
end

def combo_or_transfer?(flag)
  f = flag.to_s.downcase
  f.include?("combination") || f.include?("sex transfer") || f.include?("sex-transfer")
end

def derive_stratum(cls, flag, present)
  return "c" if combo_or_transfer?(flag)
  return "a" if real_class?(cls) && (present.nil? || present.include?(cls))
  "b"
end

# "compound seen in training" tagger (A28, G171 2026-10-21; A50 #145).
# A held-out row is tagged iff its compound names a SINGLE-compound train entry
# of train-classes.yaml, after train-name canonicalisation
# (class-labels-train-names.yaml). Match = case/punctuation-insensitive exact
# name; no hand picks. The tag lives in `flag` (appended with "; "), never in
# `stratum`. Adding only: a row already tagged but not matched aborts.
TRAIN_NAMES_FILE = File.expand_path("../../../literature/lifespan/class-labels-train-names.yaml", DIR)
SEEN_TAG = "compound seen in training"

def name_key(s)
  s.to_s.downcase.gsub(/[^a-z0-9]/, "")
end

def train_compounds
  canon = YAML.safe_load(File.read(TRAIN_NAMES_FILE)).fetch("canonical")
  YAML.safe_load(File.read(TRAIN_CLASSES_FILE))
      .fetch("labels").map { |l| l.fetch("compound") }
      .reject { |c| c.include?(" + ") }
      .map { |c| name_key(canon.fetch(c, c)) }.to_set
end

def tag_flag(flag, compound, seen_set)
  seen   = seen_set.include?(name_key(compound))
  tagged = flag.to_s.include?(SEEN_TAG)
  raise "#{compound}: tagged '#{SEEN_TAG}' but not a train compound" if tagged && !seen
  return flag if !seen || tagged
  flag.to_s.empty? ? SEEN_TAG : "#{flag}; #{SEEN_TAG}"
end

label_of = load_labels
present  = train_classes(label_of)
seen_set = train_compounds

rows = CSV.read(SPLIT, headers: true)
header = rows.headers.map { |h| h == "class_label_proposed" ? "class_label" : h }

before = Hash.new(0)
after  = Hash.new(0)
moves  = []
tagged = []
new_by = Hash.new { |h, k| h[k] = [0, 0] } # stratum => [compound-new, rows]

out = CSV.generate do |csv|
  csv << header
  rows.each do |r|
    cls  = label_of.fetch(r["compound"]) # KeyError if a compound is unlabelled
    flag = r["flag"]
    old  = r["stratum"]
    new  = derive_stratum(cls, flag, present)
    nflag = tag_flag(flag, r["compound"], seen_set)
    tagged << [r["arm_id"], r["compound"]] if nflag != flag
    new_by[new][0] += 1 unless nflag.to_s.include?(SEEN_TAG)
    new_by[new][1] += 1
    before[old] += 1
    after[new]  += 1
    moves << [r["arm_id"], r["compound"], old, new] if old != new
    csv << (header.map do |h|
      case h
      when "class_label" then cls
      when "stratum"     then new
      when "flag"        then nflag
      else r[h]
      end
    end)
  end
end

mode = ARGV.include?("--check") ? :check : :write
File.write(SPLIT, out) if mode == :write

fmt = ->(h) { %w[a b c].map { |k| "#{k} #{h[k]}" }.join(" / ") }
puts "mode: #{mode}  train-classes: #{TRAIN_CLASSES_FILE || 'PROVISIONAL (all real classes present)'}"
puts "stratum before: #{fmt.call(before)}"
puts "stratum after:  #{fmt.call(after)}"
puts "moved (#{moves.size}):"
moves.each { |a, c, o, n| puts "  #{a} #{c}: #{o} -> #{n}" }
puts "compound-new per stratum: " + %w[a b c].map { |k| "#{k} #{new_by[k][0]}/#{new_by[k][1]}" }.join(" / ")
puts "newly tagged '#{SEEN_TAG}' (#{tagged.size}):"
tagged.each { |a, c| puts "  #{a} #{c}" }
