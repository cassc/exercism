(ns run-length-encoding)

(defn- shorten [[c & more]]
  (str (when more (inc (count more))) c))

(defn- expand [[_ n c]]
  (let [n (if (seq n)
            (Integer/valueOf n)
            1)]
    (repeat n c)))

(defn run-length-encode
  "encodes a string with run-length-encoding"
  [plain-text]
  (->> plain-text
       (partition-by identity)
       (map shorten)
       (apply str)))

(defn run-length-decode
  "decodes a run-length-encoded string"
  [cipher-text]
  (->> cipher-text
       (re-seq #"(\d*)([\w|\s])")
       (mapcat expand)
       (apply str)))
