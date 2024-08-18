#!/bin/bash

# Verifica que Docker esté instalado
if ! [ -x "$(command -v docker)" ]; then
  echo "Error: Docker no está instalado." >&2
  exit 1
fi

# Función para generar un nombre aleatorio
generate_random_name() {
  echo "container-$(openssl rand -hex 3)"
}

# Crear 10 contenedores
for i in {1..10}; do
  # Generar un nombre aleatorio para el contenedor
  CONTAINER_NAME=$(generate_random_name)
  
  # Crear el contenedor utilizando la imagen alpine
  docker run -d --name "$CONTAINER_NAME" alpine sleep infinity
  
  # Verificar si el contenedor fue creado exitosamente
  if [ $? -eq 0 ]; then
    echo "Contenedor $CONTAINER_NAME creado exitosamente."
  else
    echo "Hubo un error al crear el contenedor $CONTAINER_NAME." >&2
  fi
done
