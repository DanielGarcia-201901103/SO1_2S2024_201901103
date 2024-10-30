# Usar la imagen oficial de Locust
FROM locustio/locust

# Copiar el archivo de configuración de Locust
COPY locustfile.py /mnt/locust/locustfile.py

# Establecer el directorio de trabajo
WORKDIR /mnt/locust

# Exponer los puertos utilizados por Locust
EXPOSE 8089 5557 5558
