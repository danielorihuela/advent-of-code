(ns src
  (:require [clojure.string :as string]))

(defn paths-to-current-adapter [paths-to-adapter adapter]
  (apply + (map #(get paths-to-adapter (- adapter %) 0) [1 2 3])))

(defn update-paths-to-adapter [paths-to-adapter adapter]
  (assoc paths-to-adapter adapter (paths-to-current-adapter paths-to-adapter adapter)))

(defn count-paths [adapters]
  (loop [paths-to-adapter {0 1}
         pos 0
         adapter (nth adapters pos)]
    (if (= pos (- (count adapters) 1))
      (paths-to-current-adapter paths-to-adapter adapter)
      (recur (update-paths-to-adapter paths-to-adapter adapter) (+ pos 1) (nth adapters (+ pos 1))))))

(defn -main []
  (let [adapters (sort (map read-string (string/split-lines (slurp "input.txt"))))]
    (println (count-paths (vec adapters)))))