(ns my.4.3.exec-sys-cmd
  (:require [clj-commons-exec :as exec])
  )


(def p (exec/sh ["date"]))

(deref p)
;; => {:exit 0, :out "Sat Oct 16 11:41:48 +07 2021\n", :err nil}

@(exec/sh ["date"])
;; => {:exit 0, :out "Sat Oct 16 11:41:57 +07 2021\n", :err nil}

(print (:out (clojure.java.shell/sh "ls" "-l")))
; (out) -rw-r--r--  1 earther  staff    768 Oct 12 11:59 CHANGELOG.md
; (out) -rw-r--r--  1 earther  staff  14372 Oct 12 11:59 LICENSE
; (out) -rw-r--r--  1 earther  staff    967 Oct 12 11:59 README.md
; (out) drwxr-xr-x  3 earther  staff     96 Oct 12 11:59 doc
; (out) -rw-r--r--  1 earther  staff    743 Oct 16 11:34 project.clj
; (out) drwxr-xr-x  2 earther  staff     64 Oct 12 11:59 resources
; (out) drwxr-xr-x  3 earther  staff     96 Oct 12 11:59 src
; (out) drwxr-xr-x  3 earther  staff     96 Oct 12 12:01 target
; (out) drwxr-xr-x  3 earther  staff     96 Oct 12 11:59 test

(def results (exec/sh-pipe ["cat"] ["wc" "-w"] {:in "Hello, world!"}))

@(last results)
;; => {:exit 0, :out "       2\n", :err nil}
