(ns armstrong-numbers)

(defn- pow
  ([base n]
   (pow 1 base n))
  ([result base n]
   (if (pos? n)
     (recur (*' result base) base (dec n))
     result)))

(defn armstrong? [num]
  (let [digits (str num)
        n      (count digits)]
    (->> digits
         (map #(pow (Integer/parseInt (str %)) n))
         (reduce +' 0)
         (= num))))
