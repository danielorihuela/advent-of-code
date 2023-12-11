(ns src
  (:require [clojure.string :as string]))

(defn tile-borders [tile]
  (vector
   (first tile) (apply str (map last tile)) (last tile) (apply str (map first tile))))

(defn id-tile [tile-representation]
  (let [[description tile] (string/split tile-representation #":\n")]
    (hash-map (Integer. (last (string/split description #" "))) (tile-borders (string/split-lines tile)))))

(defn ids-tiles [tiles-representation]
  (reduce conj (map id-tile (string/split tiles-representation #"\n\n"))))

(defn contiguous? [tile other-tile]
  (some true? (flatten (map #(map (fn [x] (or (= x %) (= x (apply str (reverse %))))) other-tile) tile))))

(defn number-of-contiguous-tiles [tile id-tiles]
  (- (count (filter true? (map #(contiguous? tile (last %)) id-tiles))) 1))

(defn ids-number-of-contiguous-tiles [id-tiles]
  (map #(vec [(first %) (number-of-contiguous-tiles (last %) id-tiles)]) id-tiles))

(defn -main []
  (->>
   (slurp "input.txt")
   ids-tiles
   ids-number-of-contiguous-tiles
   (filter #(= 2 (last %)))
   (map first)
   (apply *)
   println))