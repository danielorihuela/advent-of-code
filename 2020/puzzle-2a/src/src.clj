(ns src
  (:require [clojure.string :as string]))

(defn policies-and-passwords []
  (->> (slurp "input.txt")
       string/split-lines
       (map #(let [[all min max letter password] (re-find #"(\d+)-(\d+) (.): (.+)" %)]
               {:min (Integer/parseInt min)
                :max (Integer/parseInt max)
                :letter (char (first letter))
                :password password}))))

(defn valid-passwords [policies-and-passwords]
  (filter #(<= (:min %) (get (frequencies (:password %)) (:letter %) 0) (:max %)) policies-and-passwords))

(defn -main []
  (println (count (valid-passwords (policies-and-passwords)))))