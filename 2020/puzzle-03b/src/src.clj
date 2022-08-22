(ns src
  (:require [clojure.string :as string]))

(def slopes [[1 1] [1 3] [1 5] [1 7] [2 1]])

(defn slope-positions [number-of-rows number-of-columns step-height step-width]
  (let [number-of-steps (int (Math/ceil (/ number-of-rows step-height)))]
    (for [step (range 1 number-of-steps)]
      [(* step step-height) (mod (* step step-width) number-of-columns)])))

(defn number-of-trees [grid positions]
  (count (filter #(= (aget grid (first %) (last %)) \#) positions)))

(defn -main []
  (let [grid (to-array-2d (string/split-lines (slurp "input.txt")))
        number-of-rows (alength grid)
        number-of-columns (alength (aget grid 0))]
    (->> slopes
         (map #(slope-positions number-of-rows number-of-columns (first %) (last %)))
         (map #(number-of-trees grid %))
         (apply *)
         println)))