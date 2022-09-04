(ns src
  (:require [clojure.string :as string]))

(defn adjacent-seats-occupied [state y x]
  (let [combinations (filter #(not= % [y x]) (for [a [-1 0 1] b [-1 0 1]] [(+ y a) (+ x b)]))]
    (count (filter #(= % \#) (map #(get-in state %) combinations)))))

(defn next-state [old-state]
  (vec (for [y (range (count old-state))]
         (vec (for [x (range (count (first old-state)))]
                (case (get-in old-state [y x])
                  \. \.
                  \L (if (= (adjacent-seats-occupied old-state y x) 0) \# \L)
                  \# (if (>= (adjacent-seats-occupied old-state y x) 4) \L \#)))))))

(defn num-occupied-seats [state]
  (apply + (for [y (range (count state)) x (range (count (first state)))]
             (if (= \# (get-in state [y x])) 1 0))))

(defn -main []
  (loop [old-state (next-state (mapv char-array (string/split-lines (slurp "input.txt"))))
         new-state (next-state (next-state old-state))]
    (if (= 0 (compare old-state new-state)) (println (num-occupied-seats new-state))
        (recur new-state (next-state new-state)))))