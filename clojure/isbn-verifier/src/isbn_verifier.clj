(ns isbn-verifier
  (:require [clojure.string :as s]))

(def valid-chars [\0 \1 \2 \3 \4 \5 \6 \7 \8 \9 \X])
(def char->digit (zipmap valid-chars (range 0 11)))

(defn- to-digits [isbn]
  (->> (s/replace isbn "-" "")
       (map char->digit)))

(defn isbn10? [digits]
  (->> digits
       (map * (range 10 0 -1))
       (apply +)
       (#(mod % 11))
       (zero?)))

(defn isbn13? [digits]
  (->> digits
       (butlast)
       (map * (cycle [1 3]))
       (apply +)
       (#(- 10 (mod % 10)))
       (= (last digits))) )

(defn isbn? [isbn]
  (let [digits (to-digits isbn)]
    (and (every? identity digits)
         (every? #(< % 10) (butlast digits))
         (case (count digits)
           10 (isbn10? digits)
           13 (isbn13? digits)
           false))))

(defn isbn10->isbn13 [isbn]
  (let [isbn (apply str "978" (butlast (s/replace isbn "-" "")))]
    (->> isbn
         (map (fn [i c]
                (* i (char->digit c)))
              (cycle [1 3]))
         (apply +)
         (#(- 10 (mod % 10)))
         (str isbn))))
