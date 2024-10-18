package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type RequestBody struct {
	Student    string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST requests are allowed", http.StatusMethodNotAllowed)
		return
	}

	var body RequestBody
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		http.Error(w, "Invalid JSON body", http.StatusBadRequest)
		return
	}

	fmt.Printf("Student: %s, age: %d, faculty: %s, discipline: %d  \n", body.Student, body.Age, body.Faculty, body.Discipline)

	response := fmt.Sprintf("Received student: %s, age: %d, faculty: %s, discipline: %d", body.Student, body.Age, body.Faculty, body.Discipline)

	fmt.Fprintln(w, response)
}

func main() {
	http.HandleFunc("/submitAgronomia", handler)
	fmt.Println("Server running on port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
