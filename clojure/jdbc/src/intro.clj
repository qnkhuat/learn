; follow this tutorial http://clojure-doc.org/articles/ecosystem/java_jdbc/home.html
(ns jdbc.intro
  (:require [clojure.java.jdbc :as jdbc]))

(def db-spec {:classname "org.postgresql.Driver" 
              :subprotocol "postgresql"
              :subname "//localhost:5432/cookbook_exp" ;; Not needed for a non-secure local database... ;; :user "foo" :password "bar"
              })
 
; --- TEST ---
(jdbc/query db-spec ["SELECT 1*2 AS result"])
;; => ({:result 2})

; --- CREATE-TABLE ---
(def fruit-table-ddl
  (jdbc/create-table-ddl :fruit
                         [[:name "varchar(32)"]
                          [:appearance "varchar(32)"]
                          [:cost :int]
                          [:grade :real]
                          ]))
fruit-table-ddl
;; => "CREATE TABLE fruit (name varchar(32), appearance varchar(32), cost int, grade real)"

(jdbc/db-do-commands
  db-spec [fruit-table-ddl "CREATE INDEX name_ix ON fruit ( name );"])
;; => (0 0)

;(jdbc/db-do-commands
;  db-spec "DROP TABLE fruit")



; --- QUERY-DB ---
(jdbc/insert! db-spec :fruit {:name "orange" :appearance "orange" :cost 42 :grade 3})
(jdbc/insert! db-spec :fruit {:name "apple" :appearance "red" :cost 421 :grade 3})
(jdbc/insert! db-spec :fruit {:name "banana" :appearance "yellow" :cost 420 :grade 3})
;; => ({:name "banana", :appearance "yellow", :cost 420, :grade 3.0})

(jdbc/query db-spec ["SELECT * FROM fruit WHERE cost>10"])
;; => ({:name "apple", :appearance "red", :cost 421, :grade 3.0}
;;     {:name "banana", :appearance "yellow", :cost 420, :grade 3.0}
;;     {:name "orange", :appearance "orange", :cost 20, :grade 3.0})

(jdbc/update! db-spec :fruit {:cost 20} ["name=?" "orange"])
;; => (1)

(jdbc/delete! db-spec :fruit ["name=?" "apple"])
;; => (1)


; --- REDUCIBLE-QUERY ---
; the problem with jdbc/query is it retursn a fully realized result, so it'll not be effciient with 
; large results

(reduce (fn [total {:keys [cost]}] (+ total cost))
        0
        (jdbc/reducible-query db-spec
                              ["SELECT * FROM fruit WHERE cost > ?" 10]
                              {:raw? true}))
;; => 440

(transduce (map :cost)
           + ; can be called with 0, 1, or 2 arguments!
           (jdbc/reducible-query db-spec
                                 ["SELECT * FROM fruit WHERE cost > ?" 10]
                                 {:raw? true}))
;; => 440

; :: Transform a row
(jdbc/query db-spec ["SELECT * FROM fruit WHERE cost > ?" 10])
;; => ({:name "banana", :appearance "yellow", :cost 420, :grade 3.0}
;;     {:name "orange", :appearance "orange", :cost 20, :grade 3.0})


(jdbc/query db-spec ["SELECT * FROM fruit WHERE cost > ?" 10]
            {:row-fn :name})
;; => ("banana" "orange")

(jdbc/query db-spec ["SELECT * FROM fruit WHERE cost > ?" 10]
            {:row-fn :cost
             :result-set-fn (partial reduce +)})
;; => 440


(into [] (map :name) (jdbc/reducible-query db-spec ["SELECT name FROM fruit WHERE cost < ?" 50]))
;; => ["orange"]

(transduce (map :cost) + (jdbc/reducible-query db-spec ["SELECT * FROM fruit WHERE cost < ?" 50]))
;; => 20


;; --- TRANSACTION ---
(jdbc/with-db-transaction [t-con db-spec]
  (jdbc/update! t-con :fruit
                {:cost 49}
                ["grade < ?" 75])
  (jdbc/execute! t-con
                 ["update fruit set cost = ( 2 * grade ) where grade > ?" 50.0]))
;; => (0)

;; --- ERROR-HANDLING ---
(jdbc/with-db-transaction [t-con db-spec]
  (jdbc/insert-multi! t-con :fruit
                      [:name :appearance]
                      [["Grape" "yummy"]
                       ["Pear" "bruised"]])
  ;; At this point the insert! call is complete, but the transaction is
  ;; not. The exception will cause it to roll back leaving the database
  ;; untouched.
  (throw (Exception. "sql/test exception")))
































