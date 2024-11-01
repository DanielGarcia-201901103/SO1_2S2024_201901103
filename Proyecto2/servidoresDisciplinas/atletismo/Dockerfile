# Usa la imagen base de Go
FROM golang:1.22-alpine AS builder

# Crea y establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de Go y Proto
COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Compila el binario
RUN go build -o atletismo .

# Usa una imagen base más ligera para el binario
FROM alpine:latest

# Establece el directorio de trabajo
WORKDIR /app

# Copia el binario compilado desde la fase anterior
COPY --from=builder /app/atletismo .

# Expone el puerto específico
EXPOSE 8083

# Ejecuta el binario
CMD ["./atletismo"]