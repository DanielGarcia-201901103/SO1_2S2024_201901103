package main

import (
	"context"
	"log"
	"math/rand"
	"net"
	"time"

	pb "myservers/paqueteProto"

	"google.golang.org/grpc"
)

const (
	port = ":8083"
)

type server struct {
	pb.UnimplementedStudentServiceServer
}

func (s *server) SendStudent(ctx context.Context, req *pb.StudentRequest) (*pb.StudentResponse, error) {
	isWinner := determineWinner()
	status := "Winner"
	if !isWinner {
		status = "Not a Winner"
	}

	// Imprimir información recibida en lugar de enviar a Kafka
	log.Printf("Received student: %s, Age: %d, Faculty: %s, Discipline: %d", req.Student, req.Age, req.Faculty, req.Discipline)
	log.Printf("Determined status: %s", status)

	// Devolver la respuesta con el estado
	return &pb.StudentResponse{Status: status}, nil
}

// Función de probabilidad para determinar si es ganador
func determineWinner() bool {
	rand.Seed(time.Now().UnixNano())
	return rand.Intn(2) == 0
}

func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("Failed to listen on port %v: %v", port, err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterStudentServiceServer(grpcServer, &server{})
	log.Printf("Atletismo gRPC server started on port %s", port)

	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Failed to serve gRPC server Atletismo: %v", err)
	}
}
