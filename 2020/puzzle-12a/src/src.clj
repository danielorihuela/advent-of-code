(ns src
  (:require [clojure.string :as string]))

(defn move [old-position instruction]
  (let [[x y degree] old-position
        action (first instruction)
        value (Integer. (last instruction))]
    (cond
      (or (and (= action "F") (= degree 270)) (= action "N")) [x (+ y value) degree]
      (or (and (= action "F") (= degree 90)) (= action "S")) [x (- y value) degree]
      (or (and (= action "F") (= degree 0)) (= action "E")) [(+ x value) y degree]
      (or (and (= action "F") (= degree 180)) (= action "W")) [(- x value) y degree]
      (= action "L") [x y (mod (- degree value) 360)]
      (= action "R") [x y (mod (+ degree value) 360)])))

(defn -main []
  (let [instructions (map #(rest (re-matches #"(.)(\d*)" %)) (string/split-lines (slurp "input.txt")))]
    (println (apply + (map #(Math/abs %) (take 2 (reduce move [0 0 0] instructions)))))))