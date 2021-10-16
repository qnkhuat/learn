(ns my.6.1.connect-db)

(require '[clojure.java.jdbc :as jdbc]
         '[toucan.db :as db]
         '[toucan.models :as models]
         '[java-jdbc.ddl :as ddl]
         '[java-jdbc.sql :as sql])

; using java-ddl
(def db-spec {:classname "org.postgresql.Driver" 
              :subprotocol "postgresql"
              :subname "//localhost:5432/cookbook_exp" ;; Not needed for a non-secure local database... ;; :user "foo" :password "bar"
              })

; Create a table
(jdbc/db-do-commands db-spec false
                     (ddl/create-table
                       :tags
                       [:id :serial "PRIMARY KEY"]
                       [:name :varchar "NOT NULL"]))


;; => (0)


(jdbc/insert! db-spec :tags
              {:name "Clojure"}
              {:name "Java"})

(jdbc/query db-spec 
            (sql/select * :tags 
                        (sql/where {:name "Clojure"})))


; ----- Toucan ------

(db/set-default-db-connection! db-spec)
;; => {:classname "org.postgresql.Driver",
;;     :subprotocol "postgresql",
;;     :subname "//localhost:5432/cookbook_exp"}

;; define the User model
(models/defmodel Tags :tags
  models/IModel
  (models/types [this] ;; tell Toucan to automatically do Keyword <-> String conversion for :status
                {:name :keyword}))

;; => #'my.6.1.connect-db/User

(Tags 2)
;; => {:id 2, :name "Java"}

(db/insert! Tags :name "Cam") 
;; => {:id 3, :name "Cam"}
