#!/bin/bash

# Generar nombres aleatorios para los contenedores usando /dev/urandom
generate_random_name() {
    echo "contenedores-$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 8)"
}

# Construir imágenes de alto consumo
docker build -t high_cpu_mem_image -f ./consumo/Dockerfile.high ./consumo
docker build -t high_cpu_mem_image1 -f ./consumo/Dockerfile.high1 ./consumo

# Construir imágenes de bajo consumo
docker build -t low_cpu_mem_image -f ./consumo/Dockerfile.low ./consumo
docker build -t low_cpu_mem_image1 -f ./consumo/Dockerfile.low1 ./consumo

# Crear y correr 10 contenedores aleatorios (alto y bajo consumo)
for i in {1..10}; do
    container_name=$(generate_random_name)
    
    # Generar aleatoriamente si es de alto o bajo consumo
    if (( RANDOM % 2 )); then
        # Alternar entre las imágenes de alto consumo
        if (( RANDOM % 2 )); then
            docker run -d --name "$container_name" --cpus="0.3" --memory="100m" high_cpu_mem_image
            echo "Contenedor de alto consumo creado: $container_name (high_cpu_mem_image)"
        else
            docker run -d --name "$container_name" --cpus="0.3" --memory="100m" high_cpu_mem_image1
            echo "Contenedor de alto consumo creado: $container_name (high_cpu_mem_image1)"
        fi
    else
        # Alternar entre las imágenes de bajo consumo
        if (( RANDOM % 2 )); then
            docker run -d --name "$container_name" --cpus="0.2" --memory="50m" low_cpu_mem_image
            echo "Contenedor de bajo consumo creado: $container_name (low_cpu_mem_image)"
        else
            docker run -d --name "$container_name" --cpus="0.2" --memory="50m" low_cpu_mem_image1
            echo "Contenedor de bajo consumo creado: $container_name (low_cpu_mem_image1)"
        fi
    fi
done

echo "Todos los contenedores han sido creados."