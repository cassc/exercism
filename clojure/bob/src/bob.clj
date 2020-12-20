(ns bob
  (:require [clojure.string :as s]))

(defn letter? [c]
  (Character/isLetter c))

(defn upper? [c]
  (Character/isUpperCase c))

(defn yell? [s]
  (some->> s (filter letter?) seq (every? upper?)))

(defn question? [s]
  (s/ends-with? s "?"))

(defn response-for [s]
  (let [s (s/trim s)
        y (yell? s)
        q (question? s)]
    (cond
      (and y q)    "Calm down, I know what I'm doing!"
      y            "Whoa, chill out!"
      q            "Sure."
      (s/blank? s) "Fine. Be that way!"
      :else        "Whatever." )))
