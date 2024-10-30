# Usa una imagen base de Rust para compilar
FROM rust:1.72 as builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia el archivo de configuración y el código fuente
COPY Cargo.toml .
COPY src ./src

# Compila el binario de release
RUN cargo build --release

# Usa una imagen base de Debian slim para reducir el tamaño de la imagen final y asegurar la compatibilidad de bibliotecas
FROM debian:latest

# Copia el binario compilado desde la fase anterior
COPY --from=builder /app/target/release/rust_api /usr/local/bin/rust_api

# Asegura que el binario tiene permisos de ejecución
RUN chmod +x /usr/local/bin/rust_api

# Expone el puerto 8081
EXPOSE 8081

# Ejecuta el binario
CMD ["/usr/local/bin/rust_api"]
