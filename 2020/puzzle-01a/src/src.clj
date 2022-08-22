(ns src
  (:require [clojure.math.combinatorics :as combo])
  (:require [clojure.string :as string]))

(defn input-entries []
  (map #(Integer/parseInt %) (string/split-lines (slurp "input.txt"))))

(defn entries-that-sum-2020 [entries]
  (first (filter
          #(= (apply + %) 2020)
          (combo/combinations entries 2))))

(defn -main []
  (let [[a b] (entries-that-sum-2020 (input-entries))]
    (println (* a b))))