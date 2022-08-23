(ns src
  (:require [clojure.string :as string]))

(def goal-bag "shiny gold")

(defn bag-information [info]
  (let [[quantity type] (rest (re-matches #"(\d) (.*)" info))]
    {:quantity quantity :type type}))

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

(defn bag-contained? [goal-bag type rules]
  (let [childs-type (set (map #(:type %) (get rules type)))]
    (if
     (contains? childs-type goal-bag) true
     (not-every? false? (map #(bag-contained? goal-bag % rules) childs-type)))))

(defn -main []
  (let [rules (deserialize-rules (string/split (slurp "input.txt") #"\n"))]
    (println (count (filter true? (map #(bag-contained? goal-bag (first %) rules) rules))))))