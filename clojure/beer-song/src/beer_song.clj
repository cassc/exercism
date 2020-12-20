(ns beer-song
  (:require
   [clojure.string :as s]))

(defn bottles [num]
  (cond
    (> num 1)  (format "%d bottles" num)
    (pos? num) (format "%d bottle" num)
    :else      (format "no more bottles")))

(defn verse
  "Returns the nth verse of the song."
  [num]
  (if (pos? num)
    (format
      "%s of beer on the wall, %s of beer.\nTake %s down and pass it around, %s of beer on the wall.\n"
      (s/capitalize (bottles num))
      (bottles num)
      (if (= num 1) "it" "one")
      (bottles (dec num)))
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"))

(defn sing
  "Given a start and an optional end, returns all verses in this interval. If
  end is not given, the whole song from start is sung."
  ([start]
   (sing start 0))
  ([start end]
   (->> (range start (dec end) -1)
        (map verse)
        (s/join "\n"))))
