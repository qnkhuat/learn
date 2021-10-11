(ns ceaser-rot13.core
  (:gen-class)
  (:require [clojure.string :as string]  ))

(def english-alphabet
  (seq "abcdefghijklmnopqrstuvwxyz"))

(defn rot13 [alphabet text]
  (let [cipher (->> (cycle english-alphabet)
                    (drop 13)
                    (take 26)
                    (zipmap english-alphabet))]
    (string/join (replace cipher text))))

(rot13 english-alphabet "ngockq")

(defn foo
  "I don't do a whole lot."
  [& args]
  (rot13 english-alphabet (first args)))
