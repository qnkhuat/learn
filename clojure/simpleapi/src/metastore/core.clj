(ns metastore.core
  (:require 
    [compojure.core :refer [defroutes DELETE GET PUT]]
    [ring.middleware.params :refer [wrap-params]]
    [ring.middleware.session :refer [wrap-session]]
    )
  (:gen-class))

;--- Store ---

(defn new-store
  ([] (ref {}))
  ([data] (ref data))
  )

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

(defn wrap-store [handler stores]
  (fn [request]
    (handler (assoc request :stores stores))))

;--- Route ---
(defroutes routes 
  (GET "/store/:key" [key :as {stores :stores}] 
       (handle-get stores key))

  (PUT "/store/:key/:value" [key value :as {stores :stores}]
       (handle-set stores key value))

  (DELETE "/store/:key" [key :as {stores :stores}]
          (handle-del stores key))

  (PUT "/begin" [:as {stores :stores}]
       (handle-begin stores)
       "OK")

  (PUT "/commit" [:as {stores :stores}]
       (handle-commit stores)
       "OK")

  (PUT "/rollback" [:as {stores :stores}]
       (handle-rollback stores)
       "OK")

  (GET "/" []
       (str "hi"))
  )


(def app 
  (let [store (atom [(new-store)])]
    (-> routes
        wrap-params
        wrap-session
        (wrap-store store)
        )))


