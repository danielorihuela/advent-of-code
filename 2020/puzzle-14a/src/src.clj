(ns src
  (:require [clojure.string :as string]))

(defn extract-instructions [program type extract]
  (->> (map-indexed vector program)
       (filter #(re-find type (last %)))
       (map #(vec [(first %) (rest (re-find extract (last %)))]))
       (map flatten)))

(defn get-mask [masks position]
  (last (last (filter #(< (first %) position) masks))))

(defn apply-mask [mask value]
  (let [binary-value (format "%036d" (BigInteger. (Integer/toString (Integer. value) 2)))]
    (BigInteger. (apply str (map #(if (= (first %) \X) (second %) (first %)) (map vector mask binary-value))) 2)))

(defn -main []
  (let [program (string/split-lines (slurp "input.txt"))
        masks (extract-instructions program #"mask" #"mask = (.*)")
        mem-instructions (extract-instructions program #"mem" #"mem\[(.*)\] = (.*)")
        mem-addresses (set (map #(second %) mem-instructions))]
    (println (apply + (for [address mem-addresses]
                        (let [mem-instruction (last (filter #(= (second %) address) mem-instructions))]
                          (apply-mask (get-mask masks (first mem-instruction)) (last mem-instruction))))))))