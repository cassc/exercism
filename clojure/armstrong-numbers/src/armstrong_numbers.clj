(ns armstrong-numbers)

(defn- to-digits [num]
  (->> [num 0]
       (iterate (fn [[n r]] (when (pos? n) [(quot n 10) (rem n 10)])))
       (rest)
       (take-while identity)
       (mapv second)))

(defn- pow
  [base n]
  (apply * (repeat n base)))

(defn armstrong? [num]
  (let [digits (to-digits num)
        n      (count digits)]
    (->> digits
         (map #(pow % n))
         (apply +)
         (= num))))
