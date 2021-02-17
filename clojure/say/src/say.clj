(ns say
  (:require
   [clojure.string :as s]))

(def chunk-names ["" "thousand" "million" "billion" "trillion"])
(def num-names (merge
                 (zipmap
                   (range 20)
                   ["" "one" "two" "three" "four" "five" "six" "seven" "eight" "nine"
                    "ten" "eleven" "twelve" "thirteen" "fourteen" "fifteen" "sixteen" "seventeen" "eighteen" "nineteen"])
                 (zipmap
                   (range 20 100 10)
                   ["twenty" "thirty" "forty" "fifty" "sixty" "seventy" "eighty" "ninety"])))

(defn- to-chunks [num n]
  (->> num
       (iterate #(quot % n))
       (take-while pos?)
       (mapv #(rem % n))))

(defn- read-hundred [num]
  (let [h (quot num 100)]
    (when (pos? h)
      (str (num-names h) " " "hundred"))))

(defn- read-dozens [num]
  (let [n (rem num 100)]
    (or (num-names n)
        (let [[i t] (to-chunks n 10)]
          (str (num-names (* 10 t)) "-" (num-names i))))))

(defn- read-chunk [num]
  (let [h (read-hundred num)
        d (read-dozens num)]
    (str h
         (when-not (or (s/blank? d) (s/blank? h))
           " ")
         d)))

(defn number [num]
  (if (<= 0 num 999999999999)
    (if (zero? num)
      "zero"
      (->> (to-chunks num 1000)
           (map read-chunk)
           (map #(when-not (s/blank? %2)
                   (str %2 (when-not (s/blank? %1) " ") %1)) chunk-names)
           (reverse)
           (s/join " ")
           (s/trim)))
    (throw (IllegalArgumentException. "Illegal input"))))

;; Rich comment block with redefined vars ignored
#_{:clj-kondo/ignore [:redefined-var]}
(comment
  (map read-chunk (to-chunks 1928931982 1000))
  (read-chunk 0)
  (number 1000)
  (to-chunks 91 10)
  (range 21)
  (num-names 20)
  ) ;; End of rich comment block
