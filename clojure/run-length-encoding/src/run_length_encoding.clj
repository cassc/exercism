(ns run-length-encoding)

(defn run-length-encode
  "encodes a string with run-length-encoding"
  [plain-text]
  (loop [prev   nil
         cnt    0
         result ""
         text   plain-text]
    (if-let [c (first text)]
      (if (= prev c)
        (recur c (inc cnt) result (rest text))
        (recur c 1 (str result (when (> cnt 1) cnt) prev) (rest text)))
      (if prev
        (str result (when (> cnt 1) cnt) prev)
        result))))

(defn- digits-end-index [s start]
  (when (Character/isDigit (nth s start))
    (loop [idx start]
      (if (and (< idx (count s))
               (Character/isDigit (nth s idx)))
        (recur (inc idx))
        idx))))

(defn run-length-decode
  "decodes a run-length-encoded string"
  [cipher-text]
  (let [length (count cipher-text)]
    (loop [i      0
           result ""]
      (if (< i length)
        (if-let [end (digits-end-index cipher-text i)]
          (let [n (Integer/valueOf (subs cipher-text i end))]
            (println n "of" (nth cipher-text end))
            (recur (inc end) (apply str result (repeat n (nth cipher-text end)))))
          (recur (inc i) (str result (nth cipher-text i))))
        result))))


;; Rich comment block with redefined vars ignored
#_{:clj-kondo/ignore [:redefined-var]}
(comment

  (Character/digit \8 10)
  (digits-end-index "12A8B" 3)
  (run-length-decode "12WB12W3B24WB")
  (run-length-encode "zzzz ZZZ   zZ")
  (run-length-decode
    (run-length-encode "zzzz ZZZ   zZ")
    )

  ) ;; End of rich comment block
