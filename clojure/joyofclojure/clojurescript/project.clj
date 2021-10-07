(defproject joy/music "1.0.0"
  :dependencies [[org.clojure/clojure "1.8.0"]
                 [org.clojure/clojurescript "1.10.879"]]
  :plugins [[lein-cljsbuild "1.1.8"]]
  :cljsbuild
  {:builds
   [{:source-paths ["src/cljs"]
     :compiler
     {:output-to "dev-target/all.js"
      :optimizations :whitespace
      :pretty-print true}}
    {:source-paths ["src/cljs"]
     :compiler
     {:output-to "prod-target/all.js"
      :externs ["externs.js"]
      :optimizations :advanced
      :pretty-print false}}
    ]})
