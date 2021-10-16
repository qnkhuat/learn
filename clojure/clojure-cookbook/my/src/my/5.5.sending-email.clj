(ns my.5.5.sending-email)

(require '[postal.core :refer [send-message]])

(def conn {:host "smtp.gmail.com" 
           :ssl true
           :user "khuatquangngoc98@gmail.com"
           :pass "Qnkhuat.qnkhuat98gg"})

(send-message conn
              {:from "khuatquangngoc98@gmail.com"
               :to "qn.khuat@gmail.com"
               :subject "Yo bro"
               :body "it's you"
               })
