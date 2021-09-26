(ns metastore.core
  (:require 
    [compojure.core :refer [defroutes DELETE GET PUT]]
    [ring.middleware.params :refer [wrap-params]]
    [ring.middleware.session :refer [wrap-session]]
    [compojure.route :refer [not-found] ]
    )
  (:gen-class))

;--- Store ---

(defn new-store []
  (ref {}))

(defn set-store [store k v]
  (dosync (alter store assoc k v)))

(defn get-store [store k]
  (get (deref store) k))

(defn del-store [store k]
  (dosync (alter store dissoc k)))

; first update all key values from b to a
; filter a with all keys in b ( in case of delete )
(defn merge-store [a b]
  (dosync (do (alter a merge (deref b))
              (alter a #(select-keys % (keys (deref b)))))))

;--- Handlers ---

(defn handle-get [stores k]
  (let [value (get-store (last @stores) k)]
    (if (nil? value)
      "Error: key not found"
      value)))

(defn handle-set [stores k v]
  (set-store (last @stores) k v))

(defn handle-del [stores k]
  (del-store (last @stores) k))

(defn handle-commit [stores]
  (when (> (count @stores) 1)
    ((merge-store (second (reverse @stores))
                  (last @stores))
     (swap! stores #(butlast %)))))

(defn handle-rollback [stores]
  (when (> (count @stores) 1)
    (swap! stores #(butlast %))))

(defn handle-begin [stores]
  (let [transac-store (new-store)
        _ (merge-store transac-store (last @stores))]
    (swap! stores #(concat %1 [%2]) transac-store)))

(defroutes routes 
  (GET "/:key" [key] 
       (str "input key is " key))
  (not-found "<h1>Page not found</h1>")
  )

(def app 
  (-> routes
      wrap-params
      wrap-session
      ))

