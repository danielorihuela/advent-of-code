(ns src
  (:require [clojure.string :as string]))

(def passport-required-fields [:byr :iyr :eyr :hgt :hcl :ecl :pid])

(defn deserialize-passport [passport]
  (->> passport
       (#(string/split % #"[\n|\s]"))
       (map #(let [[key value] (string/split % #":")] [(keyword key) value]))
       (reduce conj {})))

(defn deserialize-passports [passports]
  (map deserialize-passport (string/split passports #"\n\n")))

(defn valid-passport [passport]
  (=
   (count passport-required-fields)
   (count (select-keys passport passport-required-fields))))

(defn -main []
  (println (count (filter valid-passport (deserialize-passports (slurp "input.txt"))))))