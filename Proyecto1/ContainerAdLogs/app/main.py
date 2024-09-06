from fastapi import FastAPI
from pydantic import BaseModel
import json
import os
from .log_handler import add_log, generate_graphs

app = FastAPI()

# Definir la ruta del archivo JSON de logs
log_file_path = "/app/logs.json"

# Modelo para recibir datos de los logs
class LogItem(BaseModel):
    timestamp: str
    level: str
    message: str

# Endpoint para recibir logs desde el servicio de Rust
@app.post("/logs/")
async def receive_log(log_item: LogItem):
    log_data = log_item.dict()
    add_log(log_data, log_file_path)
    return {"status": "Log added"}

# Endpoint para generar gráficas a partir de los logs
@app.get("/generate-graphs/")
async def generate_logs_graphs():
    generate_graphs(log_file_path)
    return {"status": "Graphs generated"}

# Endpoint para generar gráficas a partir de los logs
@app.get("/check/")
async def ok():
    return {"status": "Peticion realizada"}