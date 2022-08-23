(ns src
  (:require [clojure.string :as string]))

(defn bag-information [info]
  (let [[quantity type] (rest (re-matches #"(\d) (.*)" info))]
    {:quantity (Integer. quantity) :type type}))

(defn deserialize-rule [rule]
  (let [rule-information (string/split rule #" bags contain | bag, | bags, | bag\.| bags\.")
        root (first rule-information)
        tail (rest rule-information)
        childs (if (or (nil? tail) (= (compare (first tail) "no other") 0))
                 []
                 (vec (map bag-information tail)))]
    {root childs}))

(defn deserialize-rules [rules]
  (reduce conj {} (map deserialize-rule rules)))

(defn contained-bags [type rules]
  (let [childs (get rules type)]
    (if
     (empty? childs) 0
     (reduce + 0 (map #(+ (:quantity %) (* (:quantity %) (contained-bags (:type %) rules))) childs)))))

(defn -main []
  (let [rules (deserialize-rules (string/split (slurp "input.txt") #"\n"))]
    (println (contained-bags "shiny gold" rules))))