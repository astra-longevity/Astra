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
# in train therefore needs each train compound's class, owned by L1 (A71 is
# extending class-labels.rb to emit them, A28 option (a), G144 2026-10-21).
# Until that `train-classes` file lands, rule 2a runs with every real class
# PROVISIONALLY present in train (A28 interim instruction), so the pipeline
# runs and the a/b split is marked provisional in split-proposal.md. Point
# TRAIN_CLASSES_FILE at the committed file to switch to the final re-derive;
# the script then intersects real classes with it.
#
#   ruby join-class-labels.rb            # rewrites split.csv in place, prints counts
#   ruby join-class-labels.rb --check    # prints counts only, writes nothing

require "yaml"
require "csv"

DIR          = __dir__
SPLIT        = File.join(DIR, "split.csv")
LABELS       = File.expand_path("../../../literature/lifespan/class-labels.yaml", DIR)
# Set to a committed path once L1 delivers it; nil => provisional (all present).
TRAIN_CLASSES_FILE = nil

NON_CLASS = ["unclassified", "multi-target"].freeze

def load_labels
  doc = YAML.safe_load(File.read(LABELS))
  doc.fetch("labels").each_with_object({}) do |l, h|
    h[l.fetch("compound")] = l.fetch("class")
  end
end

def train_classes(label_of)
  return nil if TRAIN_CLASSES_FILE.nil? # provisional: every real class counts
  YAML.safe_load(File.read(TRAIN_CLASSES_FILE))
      .fetch("labels").map { |l| l.fetch("class") }.to_set
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

label_of = load_labels
present  = train_classes(label_of)

rows = CSV.read(SPLIT, headers: true)
header = rows.headers.map { |h| h == "class_label_proposed" ? "class_label" : h }

before = Hash.new(0)
after  = Hash.new(0)
moves  = []

out = CSV.generate do |csv|
  csv << header
  rows.each do |r|
    cls  = label_of.fetch(r["compound"]) # KeyError if a compound is unlabelled
    flag = r["flag"]
    old  = r["stratum"]
    new  = derive_stratum(cls, flag, present)
    before[old] += 1
    after[new]  += 1
    moves << [r["arm_id"], r["compound"], old, new] if old != new
    csv << (header.map do |h|
      case h
      when "class_label" then cls
      when "stratum"     then new
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
