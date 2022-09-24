(ns src
  (:require [clojure.string :as string]))

(defn valid-values [properties]
  (->>
   (string/split-lines properties)
   (map #(rest (re-matches #".*: (\d*)-(\d*) or (\d*)-(\d*)" %)))
   flatten
   (map read-string)
   (partition 2)
   (map #(range (first %) (+ (last %) 1)))
   flatten
   set))

(defn valid-ticket [ticket valid-values]
  (->>
   (string/split ticket #",")
   (map read-string)
   (filter #(not (contains? valid-values %)))
   (#(= (count %) 0))))

(defn valid-tickets [tickets valid-values]
  (vec (filter #(valid-ticket % valid-values) tickets)))

(defn values-inside-bounds [bounds]
  (set (flatten (map #(range (Integer. (first %)) (+ (Integer. (last %)) 1)) bounds))))

(defn compute-fields-to-valid-values [fields]
  (reduce conj (->>
                (string/split-lines fields)
                (map #(rest (re-matches #"(.*): (\d*)-(\d*) or (\d*)-(\d*)" %)))
                (map #(seq [(first %) (partition 2 (rest %))]))
                (map #(seq [(first %) (values-inside-bounds (last %))]))
                (map #(hash-map (first %) (set (last %)))))))

(defn compute-fields-to-all-positions [fields]
  (reduce conj (map #(hash-map % (set (range (count fields)))) fields)))

(defn field-invalid-positions-for-ticket [field-to-valid-values ticket-values]
  (filter some?
          (for [i (range (count ticket-values))]
            (when (not (contains? (last field-to-valid-values) (get ticket-values i))) i))))

(defn fields-invalid-positions-for-ticket [fields-to-valid-values ticket-values]
  (map #(field-invalid-positions-for-ticket % ticket-values)
       fields-to-valid-values))

(defn remove-invalid-positions-for-field [ticket fields-to-possible-positions fields-to-valid-values]
  (let [ticket-values (mapv #(Integer. %) (string/split ticket #","))
        fields-invalid-positions (fields-invalid-positions-for-ticket fields-to-valid-values ticket-values)]
    (reduce conj
            (map #(hash-map (ffirst %) (set (remove (set (last %)) (last (first %)))))
                 (map vector fields-to-possible-positions fields-invalid-positions)))))

(defn remove-invalid-positions [fields valid-nearby-tickets]
  (let [fields-to-valid-values (compute-fields-to-valid-values fields)]
    (loop [i 0
           fields-to-possible-positions (compute-fields-to-all-positions (keys fields-to-valid-values))]
      (if (= i (count valid-nearby-tickets))
        fields-to-possible-positions
        (recur (+ i 1) (remove-invalid-positions-for-field (get valid-nearby-tickets i) fields-to-possible-positions fields-to-valid-values))))))

(defn fields-to-position [fields-to-valid-positions]
  (let [fields-to-valid-positions (vec (reverse (sort-by #(count (val %)) fields-to-valid-positions)))]
    (reduce conj
            (for [i (range (- (count fields-to-valid-positions) 1))]
              (hash-map
               (first (get fields-to-valid-positions i))
               (first (remove
                       (last (get fields-to-valid-positions (+ i 1)))
                       (last (get fields-to-valid-positions i)))))))))

(defn -main []
  (let [[fields ticket nearby-tickets] (string/split (slurp "input.txt") #"\n\n")
        ticket (mapv #(Integer. %) (string/split (last (string/split-lines ticket)) #","))]
    (->>
     (valid-values fields)
     (valid-tickets (string/split-lines nearby-tickets))
     (remove-invalid-positions fields)
     fields-to-position
     (filter #(re-find #".*departure.*" (first %)))
     (map last)
     (map #(get ticket %))
     (apply *)
     println)))
