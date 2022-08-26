(ns src
  (:require [clojure.string :as string]))

(defn instructions [data]
  (map-indexed #(vec (flatten [%1 (rest (re-matches #"(.*) ([\-|\+]\d*)" %2))])) (string/split-lines data)))

(defn run-instruction [instruction]
  (case (nth instruction 1)
    "nop" [0 1]
    "acc" [(Integer. (last instruction)) 1]
    "jmp" [0 (Integer. (last instruction))]))

(defn run [instructions num-instructions]
  (loop [accumulator 0
         line 0
         runned #{}]
    (cond
      (>= line num-instructions) [accumulator 0]
      (contains? runned line) [accumulator 1]
      :else (let [[acc-inc line-inc] (run-instruction (nth instructions line))]
              (recur (+ accumulator acc-inc) (+ line line-inc) (conj runned line))))))

(defn modify-possible-corruption [instruction]
  (let [new-type (case (nth instruction 1)
                   "nop" "jmp"
                   "jmp" "nop"
                   (nth instruction 1))]
    (assoc instruction 1 new-type)))

(defn brute-force-fix-corruption [instructions]
  (let [instructions-to-modify (filterv #(re-find #"jmp|nop" (nth % 1)) instructions)]
    (map #(assoc (vec instructions) (first %) (modify-possible-corruption %)) instructions-to-modify)))

(defn -main []
  (println (ffirst (filter #(= 0 (last %)) (map #(run % (count %)) (brute-force-fix-corruption (instructions (slurp "input.txt"))))))))