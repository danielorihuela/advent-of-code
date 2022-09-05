(ns src
  (:require [clojure.string :as string]))

(defn rotate [x y degree side]
  (reduce
   #(if (= %2 "R") [(last %1) (- (first %1))] [(- (last %1)) (first %1)])
   [x y]
   (repeat (/ degree 90) side)))

(defn move [old-position instruction]
  (let [[x y x-w y-w] old-position
        action (first instruction)
        value (Integer. (last instruction))]
    (case action
      "N" [x y x-w (+ y-w value)]
      "S" [x y x-w (- y-w value)]
      "E" [x y (+ x-w value) y-w]
      "W" [x y (- x-w value) y-w]
      (or "R" "L") (let [[new-x-w new-y-w] (rotate x-w y-w value action)] [x y new-x-w new-y-w])
      "F" [(+ x (* x-w value)) (+ y (* y-w value)) x-w y-w])))

(defn -main []
  (let [instructions (map #(rest (re-matches #"(.)(\d*)" %)) (string/split-lines (slurp "input.txt")))]
    (println (apply + (map #(Math/abs %) (take 2 (reduce move [0 0 10 1] instructions)))))))