(ns run-length-encoding)

(defn run-length-encode
  "encodes a string with run-length-encoding"
  [plain-text]
  (->> plain-text
       (partition-by identity)
       (map (fn [xs]
              (let [n (count xs)
                    c (first xs)]
                (str (when (> n 1) n) c))))
       (reduce str)))

(defn run-length-decode
  "decodes a run-length-encoded string"
  [cipher-text]
  (->> cipher-text
       (re-seq #"(\d*[\w|\s])")
       (mapcat (fn [[match _]]
                 (let [c (last match)
                       n (if (> (count match) 1)
                           (Integer/valueOf (subs match 0 (dec (count match))))
                           1)]
                   (repeat n c))))
       (reduce str)))
