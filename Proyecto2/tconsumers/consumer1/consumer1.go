package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"strconv"

	"github.com/go-redis/redis/v8"
	"github.com/segmentio/kafka-go"
)

const (
	kafkaBroker = "kafka-service.kafka:9092"
	topic       = "winners"
	redisAddr   = "redis-service:6379"
)

var ctx = context.Background()

func main() {
	// Configurar conexión a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: redisAddr,
	})

	defer rdb.Close()
	// Configurar conexión a Kafka
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

		// Almacenar en Redis usando hashes
		faculty := studentInfo["faculty"].(string)
		discipline := strconv.Itoa(int(studentInfo["discipline"].(float64)))

		// Guardar el conteo de alumnos por facultad y disciplina en Redis
		rdb.HIncrBy(ctx, "winners_by_faculty", faculty, 1)
		rdb.HIncrBy(ctx, "winners_by_discipline", discipline, 1)
	}
}
