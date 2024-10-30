package main

import (
	"context"
	"log"
	"math/rand"
	"net"
	"time"

	pb "path/to/proto/student"

	"github.com/segmentio/kafka-go"
	"google.golang.org/grpc"
)

const (
	port        = ":8083"
	kafkaBroker = "localhost:9092" // Cambia esto a la dirección de tu broker de Kafka
)

type server struct {
	pb.UnimplementedStudentServiceServer
}

func (s *server) SubmitStudent(ctx context.Context, req *pb.StudentRequest) (*pb.StudentResponse, error) {
	isWinner := determineWinner()
	topic := "winners"
	if !isWinner {
		topic = "losers"
	}

	// Enviar a Kafka
	writeToKafka(topic, req.Student)

	return &pb.StudentResponse{IsWinner: isWinner}, nil
}

// Función de probabilidad para determinar si es ganador
func determineWinner() bool {
	rand.Seed(time.Now().UnixNano())
	return rand.Intn(2) == 0
}

func writeToKafka(topic, student string) {
	writer := kafka.Writer{
		Addr:  kafka.TCP(kafkaBroker),
		Topic: topic,
	}
	defer writer.Close()
	msg := kafka.Message{Value: []byte(student)}
	if err := writer.WriteMessages(context.Background(), msg); err != nil {
		log.Printf("Error al enviar a Kafka: %v", err)
	}
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
