(ns armstrong-numbers)

(defn- to-digits [num]
  (->> num
       (iterate (fn [n] (quot n 10)))
       (take-while pos?)
       (mapv (fn [n] (rem n 10)))))

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
