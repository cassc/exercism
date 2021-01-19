(ns run-length-encoding)

(defn- shorten [[c & more]]
  (cond->> c
    more (str (inc (count more)))))

(defn- expand [[_ n c]]
  (cond->> c
    (seq n) (repeat (Integer/valueOf n))))

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
       (re-seq #"(\d*)(\D)")
       (mapcat expand)
       (apply str)))
