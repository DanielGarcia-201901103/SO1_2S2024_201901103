# Usa una imagen base de Go
FROM golang:1.22-alpine AS builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia el archivo go.mod y go.sum
COPY . .

# Descarga las dependencias
RUN go mod download

# Compila la aplicación
RUN CGO_ENABLED=0 GOOS=linux go build -o consumer2 .

# Usa una imagen base más ligera para el contenedor final
FROM alpine:latest

# Instala las dependencias necesarias (si es necesario)
RUN apk --no-cache add ca-certificates

# Copia el binario desde el contenedor builder
COPY --from=builder /app/consumer2 .

# Define el comando por defecto para ejecutar la aplicación
CMD ["./consumer2"]