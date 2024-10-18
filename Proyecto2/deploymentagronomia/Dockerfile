# Usa la imagen base de Go
FROM golang:1.22-alpine

# Establece el directorio de trabajo
WORKDIR /app

# Copia el código fuente
COPY . .

# Compila el binario
RUN go build -o api .

# Expone el puerto 8080
EXPOSE 8080

# Ejecuta el binario
CMD ["./api"]
