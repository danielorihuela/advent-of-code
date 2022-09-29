(ns src
  (:require [clojure.string :as string])
  (:require [clojure.walk :as walk])
  (:gen-class))

(defn unwrap-arg [restargs]
  (if (and (list? restargs) (= 1 (count restargs)))
    (first restargs)
    restargs))

(defmacro infix [n]
  (cond
    (and (list? n) (= 1 (count n)))
    `(infix ~(first n))
    (list? n)
    (let [[arg1 op & arg2] n]
      `(~(unwrap-arg op)
        (infix ~(unwrap-arg arg1))
        (infix ~(unwrap-arg arg2))))
    :else
    n))

(defn make-sums-first [expr]
  (walk/postwalk
   #(if (list? %) (partition-by #{'*} %) %)
   expr))

(defn -main []
  (->>
   (string/split-lines (slurp "input.txt"))
   (map #(str "(" % ")"))
   (map read-string)
   (map make-sums-first)
   (map (fn [x] (apply str x)))
   (map #(str "(src/infix (" % "))"))
   (map load-string)
   (apply +)
   println
  ))

