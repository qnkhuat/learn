(ns my.7.1.ring
  (:require
    [ring.adapter.jetty :as jetty]
    clojure.pprint))


(defn handler [request]
  {:status 200
   :headers {"content-type" "text/clojure"}
   :body (with-out-str (clojure.pprint/pprint request))}
  )


(defn -main []
  (jetty/run-jetty handler {:port 3000}))
