(require '[clojure.core.match :refer [match]])

(defn simple-clojure-parser
  [expr]
  (match [expr]
         [(var :guard symbol?)] {:variable var}
         [(['fn [arg] body] :seq)] {:closure
                                    {:arg arg
                                     :body (simple-clojure-parser body)}}
         [([operator operand] :seq)] {:application
                                      {:operator (simple-clojure-parser operator)
                                       :operand (simple-clojure-parser operand)}}
         :else (throw (Exception. (str "invalid expression: " expr)))))


(simple-clojure-parser 'a)
;; => {:variable a}


(simple-clojure-parser '(fn [x] x))
;; => {:closure {:arg x, :body {:variable x}}}


(simple-clojure-parser '((fn [x] x) a))
;; => {:application
;;       {:operator {:closure {:arg x, :body {:variable x}}},
;;          :operand {:variable a}}}
