package main

import (
	"fmt"
	"log"
	"net/http"
)

func homeHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello, World!")
}

func aboutHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "About page")
}

func main() {
	mux := http.NewServeMux()

	mux.HandleFunc("/", homeHandler)
	mux.HandleFunc("/about", aboutHandler)

	log.Println("Server starting on :3333")
	if err := http.ListenAndServe(":3333", mux); err != nil {
		log.Fatal(err)
	}
}