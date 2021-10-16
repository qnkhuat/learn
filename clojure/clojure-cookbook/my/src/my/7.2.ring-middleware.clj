(ns my.7.2.ring-middleware
  (:require
    [ring.adapter.jetty :as jetty]
    [clojure.string :as str]
    clojure.pprint))

(defn parse-query-string
  [qs]
  (when (> (count qs) 0)
    (apply hash-map (str/split qs #"[&=]"))))

(defn wrap-query
  [handler]
  (fn [req]
    (let [parsed-qs (parse-query-string (:query-string req))
          new-req (assoc req :query parsed-qs)]
      (handler new-req))))


(defn handler [req]
  (let [name (get (:query req) "name")]
    {:status 200
     :body (str "Hello, " (or name "World"))}))


(defn -main []
  (jetty/run-jetty (wrap-query handler) {:port 3000}))








