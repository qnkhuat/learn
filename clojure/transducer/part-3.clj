; statefull transducer

(defn string-builder-transducer [separator]
  ; The local state *should not* be defined here.
  (fn [rf]
    ; The local state of the transducer comes here.
    (let [state (volatile! [])]
      (fn 
        ([] (rf))

        ([result] (-> result
                      (rf (apply str @state))
                      (rf)))

        ([result input]
         (let [chars @state]
           (if (= separator input)
             (do (vreset! state [])
                 (rf result (apply str chars)))
             (do (vreset! state (conj chars input))
                 result))))))))


(into []
      (string-builder-transducer 0)
      (list \H \e \l \l \o 0 \C \l \o \j \u \r \e 0 \w \o \r \l \d \!))
;; => ["Hello" "Clojure" "world!"]
