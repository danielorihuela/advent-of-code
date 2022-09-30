(ns src
  (:require [clojure.string :as string])
  (:gen-class))

(defn next-rule [rule rule-to-rules]
  (->> (string/split rule #" ")
       (map #(if (some? (re-find #"\d" %)) (get rule-to-rules %) %))
       (map #(if (and (string/includes? % "|") (> (count %) 1)) (str " ( " % " ) ") %))
       (string/join " ")))

(defn rule-string [rule rule-to-rules]
  (loop [current-rule rule]
    (if (nil? (re-find #"\d" current-rule))
      (re-pattern (string/replace current-rule " " ""))
      (recur (next-rule current-rule rule-to-rules)))))

(defn rule-to-rules-from-text [rules]
  (->> (string/split-lines rules)
       (map #(string/split % #": "))
       (map #(hash-map (first %) (string/replace (last %) #"\"" "")))
       (reduce conj)))

(defn -main []
  (let [[rules messages] (string/split (slurp "input.txt") #"\n\n")
        pattern (rule-string "0" (rule-to-rules-from-text rules))]
    (println (count (filter some? (map #(re-matches pattern %) (string/split-lines messages)))))))

