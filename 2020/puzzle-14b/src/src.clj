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
    (apply str (map #(case (first %) \0 (second %) \1 \1 \X \X) (map vector mask binary-value)))))

(defn change-floating-bit-by-possible-values [mem-address]
  (concat [(string/replace-first mem-address "X" 1)] [(string/replace-first mem-address "X" 0)]))

(defn all-possible-mem-addresses [floating-mem-address]
  (loop [possibilities [floating-mem-address]]
    (if (= (count (filter #(= % \X) (first possibilities))) 0)
      possibilities
      (recur (apply concat (map #(change-floating-bit-by-possible-values %) possibilities))))))

(defn decode-memory-address [memory-address mask]
  (map #(BigInteger. % 2) (all-possible-mem-addresses (apply-mask mask memory-address))))

(defn write-to-memory-addresses [mem-instruction masks]
  (apply conj
         (map #(hash-map % (Integer. (last mem-instruction)))
              (decode-memory-address
               (second mem-instruction)
               (get-mask masks (first mem-instruction))))))

(defn -main []
  (let [program (string/split-lines (slurp "input.txt"))
        masks (extract-instructions program #"mask" #"mask = (.*)")
        mem-instructions (extract-instructions program #"mem" #"mem\[(.*)\] = (.*)")]
    (println (apply + (vals (apply conj (map #(write-to-memory-addresses % masks) mem-instructions)))))))