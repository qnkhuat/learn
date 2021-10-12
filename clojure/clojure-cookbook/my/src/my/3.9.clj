; Building Functions with Polymorphic Behavior

;; USING defmulti
(defmulti area
  "Calculate the area of a shape"
  :type)

(defmethod area :rectangle [shape]
  (* (:length shape) (:width shape)))

(defmethod area :circle [shape]
  (* (. Math PI) (:radius shape) (:radius shape)))


(area {:type :rectangle :length 2 :width 4})
;; => 8


(area {:type :circle :radius 1})
;; => 3.141592653589793

;; USING protocol
(defprotocol Shape
  (area [s] "Calculate the area of a shape")
  (perimeter [s] "Calculate the perimeter of a shape"))

; defrecord create a persistent map
(defrecord Rectangle [length width]
  Shape
  (area [this] (* length width))
  (perimeter [this] (* 2 (+ length width))))

(->Rectangle 2 3)
;; => {:length 2, :width 3}

(area (->Rectangle 2 3))
;; => 6
(perimeter (->Rectangle 2 3))
;; => 10

; deftype create a bare-bones object
(deftype Square [length]
  Shape
  (area [this] (* length length))
  (perimeter [this] (* 4 length)))

(->Square 2)
;; => #object[my.core.Square 0x781bbe40 "my.core.Square@781bbe40"]

(area (->Square 2))
;; => 4

(perimeter (->Square 2))
;; => 8

;; Is your class modeling a domain value—thus benefiting from hash map–like function‐ality and semantics? Use defrecord.
;; Do you need to define mutable fields? Use deftype.
