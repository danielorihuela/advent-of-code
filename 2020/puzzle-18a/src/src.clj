(ns src
  (:require [clojure.string :as string])
  (:gen-class))

(defn unwrap-arg [restargs]
  (if (= 1 (count restargs))
    (first restargs)
    restargs))

(defmacro infix [n]
  (if (list? n)
    (let [[arg1 op & arg2] n]
      `(~op (infix ~arg1) (infix ~(unwrap-arg arg2))))
    n))

(defn reverse-expression [string]
  (let [chars (reverse string)]
    (apply str (map #(case % \) \( \( \) %) chars))))

(defn -main []
  (->>
   (string/split-lines (slurp "input.txt"))
   (map reverse-expression)
   (map #(load-string (str "(src/infix (" % "))")))
   (apply +)
   println))