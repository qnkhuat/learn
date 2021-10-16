(defproject my "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.10.1"]
                 [org.clojure/core.async "1.3.618"]
                 [org.clojure/core.logic "1.0.0"]
                 [clj-http "3.12.3"]
                 [http-kit "2.5.3"]
                 [com.novemberain/langohr "1.6.0"]
                 [org.clojars.hozumi/clj-commons-exec "1.0.6"]
                 [com.draines/postal "2.0.4"]
                 [toucan "1.15.4"]
                 [jline "2.11"]
                 [org.clojure/java.jdbc "0.7.0"]
                 [java-jdbc/dsl "0.1.3"]
                 [org.postgresql/postgresql "42.1.3"]
                 [criterium "0.4.6"]
                 [clj-mmap "1.1.2"]
                 [org.clojure/core.match "0.2.0"]]
  :main ^:skip-aot my.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
