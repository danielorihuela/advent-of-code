(ns src
  (:require [clojure.string :as string])
  (:require [malli.core :as malli]))

(defn deserialize-passport [passport]
  (->> passport
       (#(string/split % #"[\n|\s]"))
       (map #(let [[key value] (string/split % #":")] [(keyword key) value]))
       (reduce conj {})))

(defn deserialize-passports [passports]
  (map deserialize-passport (string/split passports #"\n\n")))

(def passport-schema
  [:and
   [:map
    [:byr :re #"^\d{4}$"]
    [:iyr :re #"^\d{4}$"]
    [:eyr :re #"^\d{4}$"]
    [:hgt :re #"\d+(cm|in)"]
    [:hcl :re #"^#[\d|a|b|c|d|e|f]{6}$"]
    [:ecl :re #"^(amb|blu|brn|gry|grn|hzl|oth)$"]
    [:pid :re #"^\d{9}$"]]
   [:fn (fn [{:keys [byr iyr eyr hgt]}]
          (and
           (<= 1920 (Integer/parseInt byr) 2002)
           (<= 2010 (Integer/parseInt iyr) 2020)
           (<= 2020 (Integer/parseInt eyr) 2030)
           (let [[_ value units] (re-matches #"(\d+)(cm|in)" hgt)
                 value (Integer/parseInt value)]
             (case units
               "cm" (<= 150 value 193)
               "in" (<= 59 value 76)))
           ))]
   ])

(defn -main []
  (println (count (filter #(malli/validate passport-schema %) (deserialize-passports (slurp "input.txt"))))))