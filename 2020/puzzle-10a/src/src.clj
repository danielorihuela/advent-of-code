(ns src
  (:require [clojure.string :as string]))

(defn diff [tuple]
  (apply (comp - -) tuple))

(defn jolts-frequencies [jolts]
  (map #(+ (last %) 1) (frequencies (map diff (partition 2 1 (sort jolts))))))

(defn -main []
  (->> (map read-string (string/split-lines (slurp "input.txt")))
       jolts-frequencies
       (apply *)
       println))