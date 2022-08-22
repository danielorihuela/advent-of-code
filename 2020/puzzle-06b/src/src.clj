(ns src
  (:require [clojure.string :as string])
  (:require [clojure.set :as set]))

(defn groups [data]
  (string/split data #"\n\n"))

(defn group-people [group]
  (string/split group #"\n"))

(defn questions-everyone-answered-with-a-yes [group]
  (apply clojure.set/intersection (map #(set (char-array %)) (group-people group))))

(defn -main []
  (->> (slurp "input.txt")
       groups
       (map #(count (questions-everyone-answered-with-a-yes %)))
       (apply +)
       println))