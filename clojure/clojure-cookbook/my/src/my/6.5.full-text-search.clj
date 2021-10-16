(ns my.6.5.full-text-search
  (:require [clucy.core :as clucy])
  )

(def index (clucy/memory-index))

(clucy/add index
           {:name "Alice" :description "Clojure expert"
            :location "North Carolina, United States"}
           {:name "Bob" :description "Clojure novice"
            :location "Berlin, Germany"}
           {:name "Eve" :description "Eavesdropper"
            :location "Maryland, United States"})


(clucy/search index "description: clojure AND location: united" 10)
;; => ({:name "Alice",
;;      :description "Clojure expert",
;;      :location "North Carolina, United States"}
;;     {:name "Alice",
;;        :description "Clojure expert",
;;          :location "North Carolina, United States"})
