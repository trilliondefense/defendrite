package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {

	http.HandleFunc("/api/health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "OK")
	})

	fs := http.FileServer(http.Dir("./frontend/build"))

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {

		path := "./frontend/build" + r.URL.Path

		_, err := os.Stat(path)

		if os.IsNotExist(err) {
			http.ServeFile(w, r, "./frontend/build/index.html")
			return
		}

		fs.ServeHTTP(w, r)
	})

	fmt.Println("Backend running on :8080")

	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		fmt.Println(err)
	}
}
