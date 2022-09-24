(ns src
  (:require [clojure.string :as string]))

(defn get-valid-values [properties]
  (->>
   (string/split-lines properties)
   (map #(rest (re-matches #".*: (\d*)-(\d*) or (\d*)-(\d*)" %)))
   flatten
   (map read-string)
   (partition 2)
   (map #(range (first %) (+ (last %) 1)))
   flatten
   set))

(defn invalid-values [nearby-tickets valid-values]
  (->>
   (string/split-lines nearby-tickets)
   rest
   vec
   (map #(string/split % #","))
   flatten
   (map read-string)
   (filter #(not (contains? valid-values %)))))

(defn -main []
  (let [text (slurp "input.txt")
        [properties _ nearby-tickets] (string/split text #"\n\n")
        valid-values (get-valid-values properties)]
    (println (apply + (invalid-values nearby-tickets valid-values)))))