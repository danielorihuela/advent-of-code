(ns src
  (:require [clojure.string :as string]))

(def directions [:north :north-east :east :south-east :south :south-west :west :north-west])

(defn occupied-seats [grid y x dir]
  (= \# (first (filter #(not= % \.) (case dir
    :north (map #(get-in grid [(- y %) x]) (range 1 (+ y 1)))
    :north-east (map #(get-in grid [(- y %) (+ x %)]) (range 1 (min (+ y 1) (- (count (first grid)) x))))
    :east (map #(get-in grid [y (+ x %)]) (range 1 (- (count (first grid)) x)))
    :south-east (map #(get-in grid [(+ y %) (+ x %)]) (range 1 (min (- (count grid) y) (- (count (first grid)) x))))
    :south (map #(get-in grid [(+ y %) x]) (range 1 (- (count grid) y)))
    :south-west (map #(get-in grid [(+ y %) (- x %)]) (range 1 (min (- (count grid) y) (+ x 1))))
    :west (map #(get-in grid [y (- x %)]) (range 1 (+ x 1)))
    :north-west (map #(get-in grid [(- y %) (- x %)]) (range 1 (min (+ y 1) (+ x 1)))))))))
    

(defn adjacent-seats-occupied [state y x]
  (count (filter true? (for [dir directions] (occupied-seats state y x dir))))
)

(defn next-state [old-state]
  (vec (for [y (range (count old-state))]
         (vec (for [x (range (count (first old-state)))]
                (case (get-in old-state [y x])
                  \. \.
                  \# (if (> (adjacent-seats-occupied old-state y x) 4) \L \#)
                  \L (if (= (adjacent-seats-occupied old-state y x) 0) \# \L)))))))

(defn num-occupied-seats [state]
  (apply + (for [y (range (count state)) x (range (count (first state)))]
             (if (= \# (get-in state [y x])) 1 0))))

(defn -main []
  (loop [old-state (next-state (mapv char-array (string/split-lines (slurp "input.txt"))))
         new-state (next-state (next-state old-state))]
    (if (= 0 (compare old-state new-state)) (println (num-occupied-seats new-state)) 
        (recur new-state (next-state new-state)))))