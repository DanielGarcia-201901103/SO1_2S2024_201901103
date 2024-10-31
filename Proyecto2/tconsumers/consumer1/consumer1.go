package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"github.com/segmentio/kafka-go"
)

const (
	kafkaBroker = "localhost:9092"
	topic       = "winners"
)

func main() {
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{kafkaBroker},
		Topic:   topic,
		GroupID: "group-winners",
	})

	defer reader.Close()

	fmt.Println("Consumer para 'winners' iniciado")

	for {
		msg, err := reader.ReadMessage(context.Background())
		if err != nil {
			log.Fatalf("Error al leer mensaje de Kafka: %v", err)
		}

		// Decodificar el mensaje
		var studentInfo map[string]interface{}
		if err := json.Unmarshal(msg.Value, &studentInfo); err != nil {
			log.Printf("Error al decodificar mensaje: %v", err)
			continue
		}

		// Imprimir la información recibida
		fmt.Printf("Ganador recibido de Kafka: %v\n", studentInfo)
	}
}
