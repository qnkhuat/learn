package main

import (
  "fmt"
  "net/http"
  "encoding/json"
  "os"
  "io/ioutil"
)

var DB_PATH = "./db.json"
var PORT = "8080"

func MapHandler(pathsToUrls map[string]string, fallback http.Handler) http.HandlerFunc {
  return func(w http.ResponseWriter, r *http.Request) {
    path := r.URL.Path
    if dest, ok := pathsToUrls[path]; ok {
      http.Redirect(w, r, dest, http.StatusSeeOther)
      return;

    }
    fallback.ServeHTTP(w, r)
  }
}

func defaultMux() *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("/", hello)
	return mux
}

func hello(w http.ResponseWriter, r *http.Request) {
  fmt.Fprintln(w, "Hello, world!")
}


func main() {

  mux := defaultMux()

  f, err := os.Open(DB_PATH)
  if err != nil {
    panic(err)
  }

  byteValue, err := ioutil.ReadAll(f)
  if err != nil {
    panic(err)
  }
  f.Close()

  var targets map[string]string
  json.Unmarshal(byteValue, &targets)

  var httpHandler = MapHandler(targets, mux)

  fmt.Println(fmt.Sprintf("Starting the serve on :%s", PORT))
  http.ListenAndServe(fmt.Sprintf(":%s", PORT), httpHandler)
}


