# Usa la imagen base de Go
FROM golang:1.22-alpine

# Crea y establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de Go y Proto
COPY . .

# Instala las dependencias
#RUN go mod download

# Compila el binario
RUN go build -o boxeo .

# Expone el puerto específico
EXPOSE 8084

# Ejecuta el binario
CMD ["./boxeo"]
