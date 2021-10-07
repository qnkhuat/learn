(require '[cljs.compiler :as comp]
         '[cljs.analyzer :as ana])

(def code-string "(defn hello [x] (js/alert (pr-str 'greetings x)))")

(def code-data (read-string code-string))

(def ast (ana/analyze (ana/empty-env) code-data))

