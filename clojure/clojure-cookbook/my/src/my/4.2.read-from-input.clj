(ns my.4.2.read-from-input
  (:import [jline.console ConsoleReader]))


(defn show-keystroke []
  (print "Enter a key: ")
  (flush)
  (let [cr (ConsoleReader.)
        keyint (.readCharacter cr)]
    (println (format "Got %d ('%c')!" keyint (char keyint)))))
