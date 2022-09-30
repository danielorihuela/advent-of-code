(ns src
  (:require [clojure.string :as string])
  (:require [instaparse.core :as insta])
  (:gen-class))

(defn -main []
  (let [[rules messages] (string/split (slurp "input.txt") #"\n\n")
        pattern (insta/parser rules)]
    (->> (string/split-lines messages)
         (map #(not= instaparse.gll.Failure (type (pattern %))))
         (filter true?)
         count
         println)))