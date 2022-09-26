(ns src
  (:require [clojure.string :as string])
  (:require [clojure.set :as set]))

(defn cartesian-product [head & tail]
  (if (= (count tail) 1)
    (apply concat (map #(map (fn [second] (flatten (seq [% second]))) (first tail)) head))
    (cartesian-product head (apply cartesian-product tail))))

(defn initial-state [initial-state-representation]
  (let [rows (string/split-lines initial-state-representation)
        num-rows (count rows)
        num-columns (count (char-array (first rows)))]
    {:active (->>
              (cartesian-product (range num-columns) (range num-rows))
              (map #(when (= (get (get rows (last %)) (first %)) \#) (flatten (seq [% 0 0]))))
              (filter some?)
              (map vec)
              set)
     :min-x -1
     :max-x (+ num-columns 1)
     :min-y -1
     :max-y (+ num-rows 1)}))

(defn neighbours [[x y z w]]
  (set
   (filter #(not= % [x y z w])
           (cartesian-product
            (range (- x 1) (+ x 2))
            (range (- y 1) (+ y 2))
            (range (- z 1) (+ z 2))
            (range (- w 1) (+ w 2))))))

(defn num-active-cubes [cubes active-cubes]
  (count (set/intersection cubes active-cubes)))

(defn next-state [min-x max-x min-y max-y min-z max-z min-w max-w active]
  (set
   (->>
    (cartesian-product
     (range min-x max-x)
     (range min-y max-y)
     (range min-z max-z)
     (range min-w max-w))
    (map
     #(if (contains? active %)
        (when (<= 2 (num-active-cubes (neighbours %) active) 3) %)
        (when (= (num-active-cubes (neighbours %) active) 3) %)))
    (filter some?))))

(defn -main []
  (let [initial-state-representation (slurp "input.txt")
        initial-state (initial-state initial-state-representation)]
    (loop [i 0
           min-x (:min-x initial-state) max-x (:max-x initial-state)
           min-y (:min-y initial-state) max-y (:max-y initial-state)
           min-z -1 max-z 2
           min-w -1 max-w 2
           active (:active initial-state)]
      (if (= i 6)
        (println (count active))
        (recur (+ i 1)
               (- min-x 1) (+ max-x 1)
               (- min-y 1) (+ max-y 1)
               (- min-z 1) (+ max-z 1)
               (- min-w 1) (+ max-w 1)
               (next-state min-x max-x min-y max-y min-z max-z min-w max-w active))))))