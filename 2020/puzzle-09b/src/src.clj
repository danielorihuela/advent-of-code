(ns src
  (:require [clojure.string :as string]))

(def window-size 25)

(defn number-list [data]
  (mapv #(Long. %) data))

(defn preamble-possible-sums [preamble]
  (set (for [x preamble y preamble] (when (not= x y) (+ x y)))))

(defn previous-numbers [numbers position]
  (subvec numbers (- position window-size) position))

(defn invalid-encrypted-number [previous-numbers current-number]
  (not (contains? (preamble-possible-sums previous-numbers) current-number)))

(defn first-invalid-number [numbers]
  (first (for [position (range window-size (count numbers))
               :let [number (nth numbers position)]
               :when (invalid-encrypted-number (previous-numbers numbers position) number)]
           number)))

(defn all-subvecs [values min-length]
  (map #(subvec values %) (range 0 (- (count values) min-length -1))))

(defn increasing-sum [values]
  (mapv #(apply + (subvec values 0 %)) (range 2 (+ (count values) 1))))

(defn encryption-weakness [numbers invalid-number]
  (let [matrix-increasing-sum (pmap increasing-sum (all-subvecs numbers 2))]
    (filter some? (for [row matrix-increasing-sum
                        :let [smallest-index (.indexOf matrix-increasing-sum row)
                              largest-index  (+ (.indexOf row invalid-number) smallest-index 1)
                              row-adding-up-to-invalid-number (subvec numbers smallest-index (+ largest-index 1))
                              smallest (apply min row-adding-up-to-invalid-number)
                              largest (apply max row-adding-up-to-invalid-number)]
                        :when (some #(= invalid-number %) row)]
                    (+ smallest largest)))))

(defn -main []
  (println (first (let [numbers (number-list (string/split-lines (slurp "input.txt")))
                        invalid-number (first-invalid-number numbers)]
                    (encryption-weakness numbers invalid-number))))
  (shutdown-agents))