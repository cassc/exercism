(ns anagram
  (:require [clojure.string :as s]))

(defn- anagram? [lcw freq candidate]
  (let [c (s/lower-case candidate)]
    (and (not= lcw c)
         (= freq (frequencies c)))))

(defn anagrams-for [word prospect-list]
  (let [lcw  (s/lower-case word)
        freq (frequencies lcw)]
    (filter (partial anagram? lcw freq) prospect-list)))
