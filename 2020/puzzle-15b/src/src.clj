(ns src
  (:require [clojure.string :as string]))

(defn next-value [value-to-turn previous-value turn]
  (- turn (get value-to-turn previous-value turn)))

(defn -main []
  (let [initial-values (map read-string (string/split (slurp "input.txt") #","))]
    (loop [turn (+ (count initial-values) 1)
           value-to-turn (into {} (map vector initial-values (range 1 (+ (count initial-values) 1))))
           previous-value (last initial-values)]
      (if (= turn 30000001)
        (println previous-value)
        (recur
         (+ turn 1)
         (assoc value-to-turn previous-value (- turn 1))
         (next-value value-to-turn previous-value (- turn 1)))))))
