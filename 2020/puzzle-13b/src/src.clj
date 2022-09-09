(ns src
  (:require [clojure.string :as string]))

(defn time-bus-ids [data]
  (let [time-bus-ids-tuples (->>
                             (string/split data #",")
                             (map-indexed #(vec [%1 %2]))
                             (filter #(not= (last %) "x"))
                             (map #(vec [(- (Integer. (last %)) (first %)) (Integer. (last %))])))]
    [(map first time-bus-ids-tuples) (map last time-bus-ids-tuples)]))

(defn -main []
  (let [[times bus-ids] (time-bus-ids (second (string/split-lines (slurp "input.txt"))))
        N (apply * bus-ids)
        n (map #(/ N %) bus-ids)
        inv (map-indexed #(.modInverse (biginteger %2) (biginteger (nth bus-ids %1))) n)]
    (println (mod (apply + (for [i (range (count bus-ids))]
                             (apply * (map #(nth % i) [times n inv])))) N))))