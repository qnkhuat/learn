(require '[clojure.core.logic :as cl]
         '[criterium.core :as criterium]
         '[clojure.core.logic.fd :as cfd])

(def movie-graph
  [;; The "Newmarket Films" studio
   [:a1 :type :FilmStudio]
   [:a1 :name "Newmarket Films"]
   [:a1 :filmsCollection :a2]
   ;; Collection of films made by Newmarket Films
   [:a2 :type :FilmCollection]
   [:a2 :film :a3]
   [:a2 :film :a6]
   ;; The movie "Memento"
   [:a3 :type :Film]
   [:a3 :name "Memento"]
   [:a3 :cast :a4]
   ;; Connects the film to its cast (actors/director/producer etc.)
   [:a4 :type :FilmCast]
   [:a4 :director :a5]
   ;; The director of "Memento"
   [:a5 :type :Person]
   [:a5 :name "Christopher Nolan"]
   ;; The movie "The Usual Suspects"
   [:a6 :type :Film]
   [:a6 :filmName "The Usual Suspects"]
   [:a6 :cast :a7]
   ;; Connects the film to its cast (actors/director/producer etc.)
   [:a7 :type :FilmCast]
   [:a7 :director :a8]
   ;; The director of "The Usual Suspects"
   [:a8 :type :Person]
   [:a8 :name "Bryan Singer"]])

(defn directors-at
  "Find all of the directors that have directed at a given studio" 
  [graph studio-name]
  (cl/run* [director-name]
           (cl/fresh [studio film-coll film cast director]
                     ;; Relate the original studio-name to a film collection 
                     (cl/membero [studio :name studio-name] graph) 
                     (cl/membero [studio :type :FilmStudio] graph) 
                     (cl/membero [studio :filmsCollection film-coll] graph)
                     ;; Relate any film collections to their individual films
                     (cl/membero [film-coll :type :FilmCollection] graph)
                     (cl/membero [film-coll :film film] graph)
                     ;; Then from film to cast members
                     (cl/membero [film :type :Film] graph)
                     (cl/membero [film :cast cast] graph)
                     ;; Grounding to cast members of type :director
                     (cl/membero [cast :type :FilmCast] graph)
                     (cl/membero [cast :director director] graph)
                     ;; Finally, attach to the director-name
                     (cl/membero [director :type :Person] graph)
                     (cl/membero [director :name director-name] graph))))

(directors-at movie-graph "Newmarket Films")


;; simple logic programming
(cl/run 1 [q]
        (cl/== 1 q))
;; => (1)


(cl/run 1 [q] 
        (cl/membero [1 q 3] [[1 2 3] [4 5 6] [7 8 9]]))
;; => (2)


(let [seq-a [["foo" 1 2] ["bar" 3 4] ["baz" 5 6]] 
      seq-b [["foo" 9 8] ["bar" 7 6] ["baz" 5 4]]]
  (cl/run 1 [q]
          (cl/fresh [first-item middle-item last-a last-b]
                    (cl/membero [first-item middle-item last-a] seq-a)
                    (cl/membero [first-item middle-item last-b] seq-b)
                    (cl/== q [last-a last-b]))))
;; => ([6 4])




; https://github.com/clojure/core.logic/wiki/A-Core.logic-Primer

; run the logic engine and return all values of q such that q is a member of the vector (1 2 3) and a member of the vector (2 3 4). 
; It will thus return (2 3), indicating that q can be either 2 or 3 and satisfy the constraints.
(cl/run* [q] ; q is called logic variable (lvar). it's also be the return of the run* function
         (cl/membero q [1 2 3]) ; there are called constraint
         (cl/membero q [2 3 4]))
;; => (2 3)

; The purpose of run is file the answer which satisfy all the constraints


; (cl/fresh [a b c] &expressions): think of it like let


; ------ Operators --------
; --- cl/== or unify ---

(cl/run* [q]
         (cl/== [1 2 3] [1 2 q]))
;; => (3)
(cl/run* [q]
         (cl/== q {:a 1 :b 2}))
;; => ({:a 1, :b 2})

; --- unification of two lvars ---
(cl/run* [q]
         (cl/membero q [1 2 3]))
;; => (1 2 3)

(cl/run* [q]
     (cl/membero q [1 2 3])
     (cl/membero q [3 4 5]))
;; => (3)

(cl/run* [q]
         (cl/fresh [a]
                   (cl/membero q [1 2 3])
                   (cl/membero a [3 4 5])
                   (cl/== q a)))
;; => (3)

; --- conde ---
; like cond  (conde &clauses). it can introduce AND and OR to the logic
(cl/run* [q]
      (cl/conde
        [(cl/== q 2) (cl/== q 1)])) ; think of it like AND
;; => ()

; return nothing bc nothing satisfy the provided logic

(cl/run* [q]
         (cl/conde
           [(cl/== q 1)] ; think of it like OR
           [(cl/== q 2)])) 
;; => (1 2)
; return both 1 and 2 because both of the cond clauses are succeed



; --- conso ---
; quite similar to cons

(conso x r s)
; a funtion succeeds only if s is a list with head x and rest r


(cl/run* [q]
         (cl/conso 1 [2 3] q))
;; => ((1 2 3))

(cl/run* [q]
         (cl/conso 1 q [1 2 3]))
;; => ((2 3))


;; --- resto ---
; constrains whatever logic variables are present such that r is (rest l)
(cl/run* [q]
      (cl/resto [1 2 3 4] q))
;; => ((2 3 4))

; ----------- Two sum --------------
(def nums (vec (range 1000)))

(def sum 1000)

(defn two-sum-logic
  [sum nums]
  (cl/run* [q]
           (cl/fresh [x y]
                     (cfd/in x y (cfd/interval (apply min nums) (apply max nums)))
                     (cfd/< x y)
                     (cfd/+ x y sum )
                     (cl/== q [x y]))))

(two-sum-logic sum nums)
;; => ([1 9] [2 8] [3 7] [4 6])

(defn two-sum-lazy-filter
  [sum nums]
  (filter #(= (reduce + %) sum) 
          (for [i (range (count nums))
                j (range (count nums))
                :when (< i j)]
            (vector (nums i) (nums j)))))

(two-sum-lazy-filter sum nums)
;; => ([1 9] [2 8] [3 7] [4 6])


(defn two-sum-cache
  [sum nums]
  (loop [temp-dict #{}
         res []
         i 0]
    (if (>= i (count nums))
      res
      (let [remaining (- sum (nums i))]
        (if (contains? temp-dict (nums i))
          (do (recur temp-dict 
                 (cons [(nums i) remaining] res)
                 (inc i)))
          (recur (conj temp-dict remaining)
                 res
                 (inc i)))))))

(two-sum-cache sum nums)
;; => ([9 1] [8 2] [7 3] [6 4])

; --- Benchmark ---
(criterium/bench (two-sum-logic sum nums))
; nums ( range 1000 )
; sum 1000
; (out) Evaluation count : 500614680 in 60 samples of 8343578 calls.
; (out)              Execution time mean : 109.958036 ns
; (out)     Execution time std-deviation : 9.962168 ns
; (out)    Execution time lower quantile : 105.354964 ns ( 2.5%)
; (out)    Execution time upper quantile : 134.000087 ns (97.5%)
; (out)                    Overhead used : 14.468425 ns
; (out) 
; (out) Found 5 outliers in 60 samples (8.3333 %)
; (out) 	low-severe	 2 (3.3333 %)
; (out) 	low-mild	 3 (5.0000 %)
; (out)  Variance from outliers : 65.2566 % Variance is severely inflated by outliers


(criterium/bench (two-sum-lazy-filter sum nums))
; nums ( range 1000 )
; sum 1000
; eval (root-form): (criterium/bench (two-sum-lazy-filter sum nums))
; (out) Evaluation count : 584636280 in 60 samples of 9743938 calls.
; (out)              Execution time mean : 89.129863 ns
; (out)     Execution time std-deviation : 1.685944 ns
; (out)    Execution time lower quantile : 87.966952 ns ( 2.5%)
; (out)    Execution time upper quantile : 94.876395 ns (97.5%)
; (out)                    Overhead used : 14.468425 ns
; (out) 
; (out) Found 5 outliers in 60 samples (8.3333 %)
; (out) 	low-severe	 2 (3.3333 %)
; (out) 	low-mild	 3 (5.0000 %)
; (out)  Variance from outliers : 7.8279 % Variance is slightly inflated by outliers

(criterium/bench (two-sum-cache sum nums))
; nums ( range 1000 )
; sum 1000
; eval (root-form): (criterium/bench (two-sum-cache sum nums))
; (out) Evaluation count : 119400 in 60 samples of 1990 calls.
; (out)              Execution time mean : 502.551583 µs
; (out)     Execution time std-deviation : 25.789349 µs
; (out)    Execution time lower quantile : 487.303832 µs ( 2.5%)
; (out)    Execution time upper quantile : 563.980068 µs (97.5%)
; (out)                    Overhead used : 14.468425 ns
; (out) 
; (out) Found 5 outliers in 60 samples (8.3333 %)
; (out) 	low-severe	 5 (8.3333 %)
; (out)  Variance from outliers : 36.8802 % Variance is moderately inflated by outliers


