; ------ Simple Request ------
(slurp "https://google.com")
;; => "<!doctype html><html itemsco...

; ------ Using http lib ------
(require '[clj-http.client :as http])

(:status (http/get "http://clojure.org"))
;; => 200


(-> (http/get "http://clojure.org")
    :headers
    (get "server"))
;; => "AmazonS3"



(-> (http/get "http://www.amazon.com/")
    :cookies
    keys)
;; => ("skin")


; ------ Perfrom async http requests ------

(require '[org.httpkit.client :as http])


(def response (http/get "http://example.com"))
;; => #'my.core/response

(:status @response)
;; => 200










