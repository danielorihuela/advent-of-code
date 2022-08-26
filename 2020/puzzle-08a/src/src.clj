(ns src
  (:require [clojure.string :as string]))

(defn instructions [data]
  (map #(rest (re-matches #"(.*) ([\-|\+]\d*)" %)) (string/split-lines data)))

(defn run-instruction [instruction]
  (case (first instruction)
    "nop" [0 1]
    "acc" [(Integer. (last instruction)) 1]
    "jmp" [0 (Integer. (last instruction))]))

(defn run [instructions]
  (loop [accumulator 0
         line 0
         runned #{}]
    (if (contains? runned line)
      accumulator
      (let [[acc-inc line-inc] (run-instruction (nth instructions line))]
        (recur (+ accumulator acc-inc) (+ line line-inc) (conj runned line))))))

(defn -main []
  (println (run (instructions (slurp "input.txt")))))