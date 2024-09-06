#!/bin/bash

# Generar nombres aleatorios para los contenedores usando /dev/urandom
generate_random_name() {
    echo "container-$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 8)"
}

# Construir imágenes de alto consumo
docker build -t high_cpu_mem_image -f ./high/Dockerfile.high ./high

# Construir imágenes de bajo consumo
docker build -t low_cpu_mem_image -f ./low/Dockerfile.low ./low

# Crear y correr contenedores de alto consumo
for i in {1..2}; do
    container_name=$(generate_random_name)
    docker run -d --name "$container_name" high_cpu_mem_image
    echo "Contenedor de alto consumo creado: $container_name"
done

# Crear y correr contenedores de bajo consumo
for i in {1..2}; do
    container_name=$(generate_random_name)
    docker run -d --name "$container_name" low_cpu_mem_image
    echo "Contenedor de bajo consumo creado: $container_name"
done

echo "Todos los contenedores han sido creados."
