(ns rna-transcription)

(def dna-rna-pairs
  {\G \C
   \C \G
   \T \A
   \A \U})

(def err (AssertionError. "Invalid input"))

(defn- dna->rna [c]
  (or (dna-rna-pairs c)
      (throw err)))

(defn to-rna [dna]
  (->> dna
       (map dna->rna)
       (apply str)))
