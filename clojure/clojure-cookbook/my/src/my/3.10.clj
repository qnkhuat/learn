; Extending a Built-In Type
(defprotocol Person
  (first-name [person])
  (last-name [person]))

(extend-type String
  Person
  (first-name [s] (first (clojure.string/split s #" ")))
  (last-name [s] (second (clojure.string/split s #" "))))


(first-name "john wick")
;; => "john"

(last-name "john wick")
;; => "wick"


;; Why use protocols when multimethods already exist? 
;; For one, speed: protocols dispatch only on the type of their first parameter. 


; same as above but extend instead of extend-type

(defn first-word [s]
  (first (clojure.string/split s #" ")))

(defn second-word [s]
  (second (clojure.string/split s #" ")))

(extend String
  Person
  {:first-name first-word
   :last-name second-word})

