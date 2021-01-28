(ns word-count
  (:require
   [clojure.string :as s]))

(defn word-count [s]
  (-> s
      (s/lower-case)
      (s/split #"\W+")
      (frequencies)))
