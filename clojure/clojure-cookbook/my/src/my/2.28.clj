; Implementing Custom Data Structures: Red-Black Trees
(require '[clojure.core.match :refer [match]])

(defn balance
  "Ensures the given subtree stays balanced by rearranging black nodes that have at least one red child and one red grandchild"
  [tree]
  (match [tree]
         [(:or ;; Left child red with left red grandchild 
               [:black [:red [:red a x b] y c] z d]
               ;; Left child red with right red grandchild 
               [:black [:red a x[:red b y c]] z d]
               ;; Right child red with left red grandchild
               [:black a x [:red [:red b y c] z d]]
               ;; Right child red with right red grandchild
               [:black a x [:red b y [:red c z d]]])] [:red [:black a x b]
                                                       y
                                                       [:black c z d]]
         :else tree))


(defn insert-val
  "Inserts x in tree.
  Returns a node with x and no children if tree is nil.
  Returned tree is balanced. See also `balance`"
  [tree x]
  (let [ins (fn ins [tree]
              (match tree
                     nil [:red nil x nil]
                     [color a y b]
                     (cond
                       (< x y) (balance [color (ins a) y b])
                       (> x y) (balance [color a y (ins b)])
                       :else tree)))
        [_ a y b] (ins tree)]
    [:black a y b]))

(defn find-val
  "Finds value x in tree"
  [tree x]
  (match tree
         nil nil
         [_ a y b] (cond
                     (< x y) (recur a x)
                     (> x y) (recur b x)
                     :else x)))



(defn- rb-tree->tree-seq
  "Return a seq of all nodes in an red-black tree." [rb-tree]
  (tree-seq sequential? (fn [[_ left _ right]]
                          (remove nil? [left right]))
            rb-tree))
(defn rb-tree->seq
  "Convert a red-black tree to a seq of its values." [rb-tree]
  (map (fn [[_ _ val _]] val) (rb-tree->tree-seq rb-tree)))

(rb-tree->seq (-> nil
                  (insert-val 5)
                  (insert-val 2)))
;; => (5 2)

(deftype RedBlackTree [tree] 
  clojure.lang.IPersistentSet
  (cons [self v] (RedBlackTree. (insert-val tree v))) 
  (empty [self] (RedBlackTree. nil))
  (equiv [self o] (if (instance? RedBlackTree o)
                    (= tree (.tree o))
                    false))
  (seq [this] (if tree (rb-tree->seq tree) nil))
  (get [this n] (find-val tree n))
  (contains [this n] (boolean (get this n)))
  ;; (disjoin [this n] ...) ;; Omitted due to complexity 

  clojure.lang.IFn
  (invoke [this n] (get this n))

  Object
  (toString [this] (pr-str this)))

(defmethod print-method RedBlackTree [o ^java.io.Writer w] 
  (.write w (str "#rbt " (pr-str (.tree o)))))


(into (->RedBlackTree nil) (range 2))
;; => #{0 1}

(def to-ten (into (->RedBlackTree nil) (range 10)))

(seq to-ten)
;; => (3 1 0 2 5 4 7 6 8 9)

(get to-ten 9)
;; => 9

(contains? to-ten 9)
;; => true
