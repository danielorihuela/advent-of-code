(ns src
  (:require [clojure.string :as string]))

(defn bus-ids [data]
  (map #(Integer. %) (filter #(not= % "x") (string/split data #","))))

(defn earliest-departures [bus-ids min-departure-time]
  (map #(vec [% (+ % (* % (int (/ min-departure-time %))))]) bus-ids))

(defn result [[bus-id earliest-departure] estimated-earliest-departure]
  (* bus-id (- earliest-departure estimated-earliest-departure)))

(defn -main []
  (let [[first-line second-line] (string/split-lines (slurp "input.txt"))
        estimated-earliest-departure (Integer. first-line)
        bus-ids (bus-ids second-line)]
    (println (result
              (apply min-key second (earliest-departures bus-ids estimated-earliest-departure))
              estimated-earliest-departure))))