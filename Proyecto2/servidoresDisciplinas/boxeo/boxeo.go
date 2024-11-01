package main

import (
	"context"
	"encoding/json"
	"log"
	"math/rand"
	"net"
	"time"

	pb "myservers1/paqueteProto"

	"github.com/segmentio/kafka-go"
	"google.golang.org/grpc"
)

const (
	port        = ":8084"
	kafkaBroker = "kafka-service.kafka:9092"
)

type server struct {
	pb.UnimplementedStudentServiceServer
}

func (s *server) SendStudent(ctx context.Context, req *pb.StudentRequest) (*pb.StudentResponse, error) {
	isWinner := determineWinner()
	topic := "winners"
	if !isWinner {
		topic = "losers"
	}

	// Imprimir información recibida en lugar de enviar a Kafka
	//log.Printf("Received student: %s, Age: %d, Faculty: %s, Discipline: %d", req.Student, req.Age, req.Faculty, req.Discipline)
	//log.Printf("Determined status: %s", status)

	studentInfo := map[string]interface{}{
		"student":    req.Student,
		"age":        req.Age,
		"faculty":    req.Faculty,
		"discipline": req.Discipline,
	}
	message, err := json.Marshal(studentInfo)
	if err != nil {
		log.Printf("Error al codificar el mensaje: %v", err)
		return &pb.StudentResponse{Status: "Encoding Error"}, err
	}

	// Enviar el mensaje a Kafka
	err = writeToKafka(topic, message)
	if err != nil {
		return &pb.StudentResponse{Status: "Kafka Error"}, err
	}

	status := "winners"
	if !isWinner {
		status = "Not a Winner"
	}

	log.Printf("Sent to Kafka - Student: %s, Age: %d, Faculty: %s, Discipline: %d, Status: %s", req.Student, req.Age, req.Faculty, req.Discipline, status)

	// Devolver la respuesta con el estado
	return &pb.StudentResponse{Status: status}, nil
}

// Función de probabilidad para determinar si es ganador
func determineWinner() bool {
	rand.Seed(time.Now().UnixNano())
	return rand.Intn(2) == 0
}

func writeToKafka(topic string, message []byte) error {
	writer := kafka.Writer{
		Addr:     kafka.TCP(kafkaBroker),
		Topic:    topic,
		Balancer: &kafka.LeastBytes{},
	}
	defer writer.Close()

	err := writer.WriteMessages(context.Background(), kafka.Message{Value: message})
	if err != nil {
		log.Printf("Error al enviar a Kafka: %v", err)
	}
	return err
}

func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("Failed to listen on port %v: %v", port, err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterStudentServiceServer(grpcServer, &server{})
	log.Printf("Boxeo gRPC server started on port %s", port)

	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Failed to serve gRPC server Boxeo: %v", err)
	}
}
