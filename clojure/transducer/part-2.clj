; simple one-one mapping

(def inc-tranducer
  (fn [rf]
    ; init
    (println "rf: " rf)
    (fn 
      ; init
      ([] (rf))
      ; completion
      ([result] (rf result))
      ; process
      ([result input] (rf result (inc input))))))


(into [] (map inc) [1 2 3])
;; => [2 3 4]

(into [] inc-tranducer [1 2 3]) ; in this case rf will be conj!
;; => [2 3 4]

(transduce inc-tranducer
           conj [] [1 2 3])
;; => [2 3 4]

(defn add-transducer [n]
  (fn [rf]
    (fn ([] (rf))
      ([result] (rf result))
      ([result input] (rf result (+ input n))))))

(into [] (add-transducer 3) (list 4 5 6))
;; => [7 8 9]


; transducer that producer 2 output give one
(defn glitch-transducer [animal]
  (fn [rf]
    (fn ([] (rf))
      ([result] (rf result))
      ([result input]
       (if (= animal input)
         (-> result
             (rf input)
             (rf input)) ; Send the input twice to the output pipeline.
         (rf result input))))))


(into [] (glitch-transducer :cat) (list :dog :cat :lynel))
;; => [:dog :cat :cat :lynel]


























