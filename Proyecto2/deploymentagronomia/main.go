package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	pb "myapi/paqueteProto" // Importa el paquete generado

	"google.golang.org/grpc"
)

type RequestBody struct {
	Student    string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func sendToServer(body RequestBody, address string) {
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		log.Printf("Could not connect to gRPC server: %v", err)
		return
	}
	defer conn.Close()

	client := pb.NewStudentServiceClient(conn) // Cambia a NewStudentServiceClient
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	req := &pb.StudentRequest{
		Student:    body.Student,
		Age:        int32(body.Age),
		Faculty:    body.Faculty,
		Discipline: int32(body.Discipline),
	}

	_, err = client.SendStudent(ctx, req)
	if err != nil {
		log.Printf("Could not send student: %v", err)
	}
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

	fmt.Printf("Student: %s, age: %d, faculty: %s, discipline: %d\n", body.Student, body.Age, body.Faculty, body.Discipline)

	// Determinar el servidor gRPC según la disciplina
	var address string
	switch body.Discipline {
	case 1:
		address = "natacion-service:8082" // Natación
	case 2:
		address = "atletismo-service:8083" // Atletismo
	case 3:
		address = "boxeo-service:8084" // Boxeo
	default:
		http.Error(w, "Invalid discipline", http.StatusBadRequest)
		return
	}

	// Enviar al servidor correspondiente usando una goroutine
	go sendToServer(body, address)

	response := fmt.Sprintf("Received student: %s, age: %d, faculty: %s, discipline: %d", body.Student, body.Age, body.Faculty, body.Discipline)
	fmt.Fprintln(w, response)
}

func main() {
	http.HandleFunc("/submitAgronomia", handler)
	fmt.Println("Server running on port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
