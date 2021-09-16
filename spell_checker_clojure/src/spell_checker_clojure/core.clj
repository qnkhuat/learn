(ns spell-checker-clojure.core
  (:require [clojure.string :as str])
  (:import (org.apache.commons.lang3 StringUtils)))

(def words 
  (set (map str/trim (set (str/split-lines (slurp "./resources/wordsEn.txt"))))))

(defn correct? [word] 
  (contains? words word))

(defn distance [a b]
  (StringUtils/getLevenshteinDistance a b))

(defn min-distance [word]
  (apply min-key (partial distance word) words))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [word (first args)]
    (if (correct? word)
      (print "correct\n")
      (println "Did you mean" (min-distance word) "?")
      )))
