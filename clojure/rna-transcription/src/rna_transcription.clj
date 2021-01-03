(ns rna-transcription)

(def dna-rna-pairs
  {\G \C
   \C \G
   \T \A
   \A \U})

(defn to-rna [dna]
  {:post [(= (count %) (count dna))]}
  (->> dna
       (map dna-rna-pairs)
       (apply str)))
