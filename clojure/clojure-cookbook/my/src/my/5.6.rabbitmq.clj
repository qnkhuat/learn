(ns my.5.6.rabbitmq)

(require 'langohr.core
         'langohr.channel)

(clojure.java.shell/sh "rabbitmq-server")

;; Connect to local RabbitMQ cluster node on localhost:5672
(def conn (langohr.core/connect {:hostname "localhost"}))
