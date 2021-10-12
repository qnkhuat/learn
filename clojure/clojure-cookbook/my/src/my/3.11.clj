; Decoupling Consumers and Producers with core.async


(defn database-consumer
  "Accept messages and persist them to a database."
  [msg]
  (println (format "database-consumer received message %s" msg)))

(defn sse-consumer
  "Accept messages and pass them to web browsers via SSE." 
  [msg]
  (println (format "sse-consumer received message %s" msg)))

(defn messages
  "Fetch messages from Twitter." []
  (range 4))


(defn message-producer
  [& consumers]
  (doseq [msg (messages)
          consumer consumers]
    (consumer msg)))


(message-producer database-consumer sse-consumer)
; (out) database-consumer received message 0
; (out) sse-consumer received message 0
; (out) database-consumer received message 1
; (out) sse-consumer received message 1
; (out) database-consumer received message 2
; (out) sse-consumer received message 2
; (out) database-consumer received message 3
; (out) sse-consumer received message 3


(require '[clojure.core.async :refer [chan sliding-buffer go
                                      go-loop timeout >! <!]])

(defn database-consumer
  []
  (let [in (chan (sliding-buffer 64))]
    (go-loop [data (<! in)]
             (when data
               (println (format "database-consumder received data %s" data)))
             (recur (<! in)))))

(defn sse-consumer
  []
  (let [in (chan (sliding-buffer 64))]
    (go-loop [data (<! in)]
             (when data
               (println (format "sse-consumer received data %s" data)))
             (recur (<! in)))))

(defn producer
  "Produce messages and deliver them to consumers." [& channels]
  (go
    (doseq [msg (messages)
            out  channels]
      (<! (timeout 100))
      (>! out msg))))

(producer (database-consumer) (sse-consumer))

; USING async
