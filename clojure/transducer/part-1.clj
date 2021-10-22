; default into
(into [] {:a 1, :b 2})
;; => [[:a 1] [:b 2]]

; (map identity) is a transducer, it works with into because into is designed to work with transducer
(into ["b" "a"] (map identity) (list "H" "e" "l" "l" "o"))
;; => ["b" "a" "H" "e" "l" "l" "o"]

(into ["b" "a"]
      (map #(char (dec (.charCodeAt % 0))))
      (list "u" "n" "b" "o"))
;; => ["b" "a" "t" "m" "a" "n"]

(into []
      (filter #(<= (.charCodeAt "a" 0) (.charCodeAt % 0) (.charCodeAt "f" 0)))
      (list "c" "r" "a" "f" "e" "b" "h" "a" "b" "l" "e"))
;; => ["c" "a" "f" "e" "b" "a" "b" "e"]

(into []
      (mapcat #(if (<= 0 % 9)
                 (list % %)
                 (list %)))
      (list 10 5 16 7 13))
;; => [10 5 5 16 7 7 13]


; mapcat
(into [] (mapcat #(repeat 3 %) ["a" "b" "c"]))
;; => ["a" "a" "a" "b" "b" "b" "c" "c" "c"]


; combine multiple transducer
(into []
      (comp
       (map inc)                 ; first step
       (filter odd?)             ; second step
       (mapcat #(if (<= 0 % 9)   ; third step
                  (list % %)
                  (list %))))
      (list 8 9 10 11 12))
;; => [9 9 11 13]












































