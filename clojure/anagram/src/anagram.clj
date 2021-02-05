(ns anagram
  (:require [clojure.string :as s]))

(defn anagrams-for [word prospect-list]
  (let [word (s/lower-case word)
        w    (frequencies word)]
    (filterv #(let [candidate (s/lower-case %)]
                (and (not (= word candidate))
                     (= w (frequencies candidate))))
             prospect-list)))
