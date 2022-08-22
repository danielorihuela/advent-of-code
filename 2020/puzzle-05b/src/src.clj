(ns src
  (:require [clojure.string :as string]))

(def min-row 0)
(def max-row 127)
(def min-column 0)
(def max-column 7)

(def character-to-half
  {\F :lower \L :lower
   \B :upper \R :upper})

(defn narrow-down [[lower-bound upper-bound] which-half]
  (let [mid-point (int (Math/ceil (/ (- upper-bound lower-bound) 2)))]
    (case which-half
      :lower [lower-bound (- upper-bound mid-point)]
      :upper [(+ lower-bound mid-point) upper-bound])))

(defn binary-search [lower-bound upper-bound halves-sequence]
  (first (reduce #(narrow-down %1 %2) [lower-bound upper-bound] halves-sequence)))

(defn compute-seat-id [seat-description]
  (let [[_ row-description column-description] (map #(char-array %) (re-find #"^([a-zA-Z]{7})([a-zA-Z]{3})$" seat-description))
        row (binary-search min-row max-row (map #(get character-to-half %) row-description))
        column (binary-search min-column max-column (map #(get character-to-half %) column-description))]
    (+ (* row 8) column)))

(defn seats-id [seats-descriptions]
  (map #(compute-seat-id %) seats-descriptions))

(defn missing-seat-id
  ([sorted-seats-id]
   (missing-seat-id 0 (count sorted-seats-id) sorted-seats-id))
  ([lower-bound upper-bound sorted-seats-id]
   (let [first-seat (nth sorted-seats-id 0)
         number-of-seats (- upper-bound lower-bound)
         mid-point (+ lower-bound (int (Math/ceil (/ number-of-seats 2))))]
     (if (= (nth sorted-seats-id mid-point) (+ first-seat mid-point))
       (recur mid-point upper-bound sorted-seats-id)
       (if (not= lower-bound (- upper-bound 1))
         (recur lower-bound mid-point sorted-seats-id)
         (+ lower-bound first-seat 1))))))

(defn -main []
  (println (missing-seat-id (sort (seats-id (string/split (slurp "input.txt") #"\n"))))))
