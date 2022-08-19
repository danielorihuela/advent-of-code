(ns src
  (:require [clojure.string :as string]))

(defn groups [data]
  (string/split data #"\n\n"))

(defn group-people [group]
  (string/split group #"\n"))

(defn unique-questions-answered-with-a-yes [group]
  (set (char-array (apply str (group-people group)))))

(defn -main []
  (->> (slurp "input.txt")
       groups
       (map #(count (unique-questions-answered-with-a-yes %)))
       (apply +)
       println))