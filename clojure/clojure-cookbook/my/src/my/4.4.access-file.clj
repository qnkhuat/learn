(ns my.4.4.access-file
  (:require [clojure.java.io :as io]
            [clojure.edn :as edn]))


(->> "4.4.access-file.edn"
     io/resource
     slurp
     edn/read-string)
;; => [{:first-name "John", :last-name "McCarthy", :language "Lisp"}
;; {:first-name "Guido", :last-name "Van Rossum", :language "Python"}
;; {:first-name "Rich", :last-name "Hickey", :language "Clojure"}]

(->> "4.4.access-file.edn"
     io/resource
     slurp
     edn/read-string
     (map :language)
     )
;; => ("Lisp" "Python" "Clojure")



;; ------- Copy file ---------

(clojure.java.io/copy
  (clojure.java.io/file "./src/my/3.10.clj")
  (clojure.java.io/file "./my-new-copy.txt"))
;; => nil

;; ------- List file ---------

(clojure.java.io/file "./src/my")
;; => #object[java.io.File 0x4a6d056c "./src/my"]

(file-seq (clojure.java.io/file "./src/my"))
;; => (#object[java.io.File 0x4331b4d1 "./src/my"]
;; #object[java.io.File 0x47fcf38d "./src/my/3.12.clj"]
;; #object[java.io.File 0x1295f07 "./src/my/4.3.exec-sys-cmd.clj"]
;; #object[java.io.File 0x525e0238 "./src/my/3.13.clj"]
;; #object[java.io.File 0x204156f "./src/my/3.11.clj"]
;; #object[java.io.File 0x7587d14c "./src/my/4.4.access-file.clj"]
;; #object[java.io.File 0x3e1d799c "./src/my/3.10.clj"]
;; #object[java.io.File 0x1cf0e3a5 "./src/my/4.2.read-from-input.clj"]
;; #object[java.io.File 0x3efd2239 "./src/my/2.27.clj"]
;; #object[java.io.File 0x753b891b "./src/my/3.9.clj"]
;; #object[java.io.File 0x20bfcfb9 "./src/my/2.22.clj"]
;; #object[java.io.File 0x3bc9edc0 "./src/my/core.clj"]
;; #object[java.io.File 0x6c72fc6a "./src/my/2.28.clj"])


;; ------- memmap a file ---------


(require '[clj-mmap :as mmap])

(with-open [file (mmap/get-mmap (io/resource "4.4.access-file.edn"))] 
  (let [n-bytes 10
        file-size     (.size file)
        first-n-bytes (mmap/get-bytes file 0 n-bytes)
        last-n-bytes  (mmap/get-bytes file (- file-size n-bytes) n-bytes)]
    [(String. first-n-bytes "UTF-8")
     (String. last-n-bytes  "UTF-8")]))


;; ------- write to a file ---------

(spit "resources/4.4.out.txt" "ngockq" :encoding "UTF-8")

(spit "resources/4.4.out.txt" "ngockq" :encoding "UTF-8")

(with-open [w (clojure.java.io/writer "resources/4.4.out.txt")]
  (doseq [line ["ngockq" "dep trai"]]
    (.write w line)
    (.newLine w)))

; append
(spit (io/resource "4.4.out.txt") "still-deptrai" :encoding "UTF-8" :append true)

(slurp (io/resource "4.4.out.txt"))
;; => "ngockq\ndep trai\nstill-deptrai"


;; ------- Read and write clojure data ---------
(spit "resources/data.clj" (pr-str [:a :b :c]) :encoding "UTF-8" )


(read-string (slurp (io/resource "data.clj")))
;; => [:a :b :c]






















