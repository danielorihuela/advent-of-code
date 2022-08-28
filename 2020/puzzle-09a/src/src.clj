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

(defn -main []
  (-> (let [numbers (number-list (string/split-lines (slurp "input.txt")))]
        (for [position (range window-size (count numbers))
              :when (invalid-encrypted-number (previous-numbers numbers position) (nth numbers position))]
          (nth numbers position)))
      first
      println))