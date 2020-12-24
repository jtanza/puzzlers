(ns puzzler-2020.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def graph (apply merge (map (fn [[x & xs]] {x (set xs)}) ["byrtlo" "rbcleo" "chrego" "hcsgwo" "nyswot" "wngosh"
                                                          "syhnwo" "ysbtno" "letobr" "tnlyob" "elrcog" "goechw"
                                                          "obrchsylegwnt"])))

(defn has-path?
  "Determines if a string-like `path` (e.g. \"boy\") exists in our [[graph]]."
  [path]
  (every? (fn [[a b]] (contains? (graph a) b)) (partition 2 1 path)))

(defn meets-criteria?
  "Ensures that our `word` is comprised solely of characters from our [[graph]],
  none of which repeat sequentially."
  [word]
  (and (empty? (set/difference (set word) (set (keys graph))))
       (not-any? #(seq (rest %)) (partition-by identity word))))

(defn read-words
  [file]
  (->> file
       slurp
       str/lower-case
       str/split-lines))

(defn -main
  [& args]
  (println (filter has-path? (filter meets-criteria? (read-words (first args))))))

