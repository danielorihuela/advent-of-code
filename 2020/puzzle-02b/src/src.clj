(ns src
  (:require [clojure.string :as string]))

(defn policies-and-passwords []
  (->> (slurp "input.txt")
       string/split-lines
       (map #(let [[all first-pos second-pos letter password] (re-find #"(\d+)-(\d+) (.): (.+)" %)]
               {:first-pos (- (Integer/parseInt first-pos) 1)
                :second-pos (- (Integer/parseInt second-pos) 1)
                :letter (char (first letter))
                :password password}))))

(defn valid-passwords [policies-and-passwords]
  (filter
   #(or
     (and (= (nth (:password %) (:first-pos %)) (:letter %)) (not= (nth (:password %) (:second-pos %)) (:letter %)))
     (and (not= (nth (:password %) (:first-pos %)) (:letter %)) (= (nth (:password %) (:second-pos %)) (:letter %))))
   policies-and-passwords))

(defn -main []
  (println (count (valid-passwords (policies-and-passwords)))))