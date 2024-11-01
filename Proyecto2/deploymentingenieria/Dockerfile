# Usa una imagen base de Rust para compilar
FROM rust:1.72 as builder

# Instala cmake
RUN apt-get update && apt-get install -y cmake

# Establece el directorio de trabajo
WORKDIR /app

# Copia el archivo de configuración y el código fuente
COPY Cargo.toml .
COPY src ./src
COPY build.rs .
COPY proto ./proto
# Compila el binario de release
RUN cargo build --release

# Usa una imagen base de Debian slim para reducir el tamaño de la imagen final y asegurar la compatibilidad de bibliotecas
FROM debian:latest

# Copia el binario compilado desde la fase anterior
COPY --from=builder /app/target/release/cliente-rust /usr/local/bin/cliente-rust

# Asegura que el binario tiene permisos de ejecución
RUN chmod +x /usr/local/bin/cliente-rust

# Expone el puerto 8081
EXPOSE 8081

# Ejecuta el binario
CMD ["/usr/local/bin/cliente-rust"]
