import json
import os
import matplotlib.pyplot as plt
import pandas as pd

def add_log(log_data, log_file_path):
    # Cargar logs existentes
    if os.path.exists(log_file_path):
        with open(log_file_path, 'r') as file:
            logs = json.load(file)
    else:
        logs = []

    # Agregar el nuevo log
    logs.append(log_data)

    # Guardar los logs actualizados en el archivo JSON
    with open(log_file_path, 'w') as file:
        json.dump(logs, file, indent=4)

def generate_graphs(log_file_path):
    # Cargar los logs en un DataFrame de pandas
    if os.path.exists(log_file_path):
        with open(log_file_path, 'r') as file:
            logs = json.load(file)
            df = pd.DataFrame(logs)
    else:
        return

    # Gráfica 1: Distribución de niveles de log
    plt.figure(figsize=(10, 6))
    df['level'].value_counts().plot(kind='bar', color='skyblue')
    plt.title('Distribución de Niveles de Log')
    plt.xlabel('Nivel')
    plt.ylabel('Cantidad')
    plt.savefig('/app/level_distribution.png')

    # Gráfica 2: Logs por tiempo
    plt.figure(figsize=(10, 6))
    df['timestamp'] = pd.to_datetime(df['timestamp'])
    df.groupby(df['timestamp'].dt.hour).size().plot(kind='line', color='green')
    plt.title('Logs por Hora')
    plt.xlabel('Hora del Día')
    plt.ylabel('Cantidad de Logs')
    plt.savefig('/app/logs_per_hour.png')
