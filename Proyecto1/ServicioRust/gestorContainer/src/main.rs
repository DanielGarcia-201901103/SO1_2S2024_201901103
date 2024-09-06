/*use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;
use chrono::Utc;

#[derive(Serialize, Debug)]  // Agregado Debug aquí
struct LogItem {
    timestamp: String,
    level: String,
    message: String,
}

#[derive(Deserialize)]
struct StatusResponse {
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Crear cliente HTTP
    let client = Client::new();

    // Dirección del contenedor de Python (asegúrate de que los puertos estén correctamente configurados)
    let python_container_url = "http://localhost:8000";

    // Crear un log de prueba
    let log_item = LogItem {
        timestamp: Utc::now().to_rfc3339(),
        level: "INFO".into(),
        message: "Este es un log de prueba".into(),
    };

    // Enviar log
    let response = client
        .post(&format!("{}/logs/", python_container_url))
        .json(&log_item)  // Aquí estamos enviando el JSON directamente
        .send()
        .await?;

    if response.status().is_success() {
        println!("Log enviado: {}", log_item.message);
    } else {
        println!("Error al enviar log: {:?}", response.status());
        println!("Enviando log: {:?}", log_item); // Ahora log_item implementa Debug
    }

    // Generar gráficas
    let response = client
        .get(&format!("{}/generate-graphs/", python_container_url))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Gráficas generadas con éxito.");
    } else {
        println!("Error al generar gráficas: {:?}", response.status());
    }

    // Realizar petición al endpoint /check/
    let response = client
        .get(&format!("{}/check/", python_container_url))
        .send()
        .await?;

    if response.status().is_success() {
        // Deserializar respuesta JSON
        let status_response: StatusResponse = response.json().await?;
        println!("Respuesta del endpoint /check/: {}", status_response.status);
    } else {
        println!("Error en la petición al endpoint /check/: {:?}", response.status());
    }

    Ok(())
}*/
use std::process::Command;
//use std::env;
use std::io;
use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::Client;
use tokio;
use chrono::Utc;
use std::{thread, time::Duration};


// Definir estructura para deserializar la respuesta del endpoint /check/
#[derive(Deserialize)]
struct StatusResponse {
    status: String,
}

fn ejecutar_comando() -> std::io::Result<()> {
    //Verificar el directorio actual
    //let current_dir = env::current_dir()?;
    //println!("Directorio actual: {:?}", current_dir);
    // Definir la ruta del directorio donde quieres ejecutar el comando
    /*let ruta_comando = "/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs";  // Ruta relativa desde el directorio actual

    // Crear el comando docker-compose con la opción de cambiar el directorio
    let status = Command::new("/snap/bin/docker-compose")

        .arg("up")
        .arg("--build")
        .current_dir(ruta_comando)  // Cambia al directorio objetivo
        .status()?;  // Ejecuta el comando y espera el estado de salida

    // Verificar si el comando se ejecutó correctamente
    if status.success() {
        println!("El comando se ejecutó exitosamente.");
    } else {
        println!("Hubo un error al ejecutar el comando.");
    }

    Ok(())*/
    Command::new("gnome-terminal")
        .arg("--")
        .arg("bash")
        .arg("-c")
        .arg("cd /home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs && docker-compose up --build; exec bash")
        .spawn()?;  // Usamos spawn en lugar de status para no esperar a que termine

    Ok(())
}

// Función asíncrona para verificar el endpoint /check/
async fn verificar_endpoint() -> Result<(), Box<dyn Error>> {
    // Crear cliente HTTP
    let client = Client::new();

    // Dirección del contenedor de Python
    let python_container_url = "http://localhost:8000";

    // Realizar petición al endpoint /check/
    let response = client
        .get(&format!("{}/check/", python_container_url))
        .send()
        .await?;

    if response.status().is_success() {
        // Deserializar respuesta JSON
        let status_response: StatusResponse = response.json().await?;
        println!("Respuesta del endpoint /check/: {}", status_response.status);
    } else {
        println!("Error en la petición al endpoint /check/: {:?}", response.status());
    }

    Ok(())
}
// El main debe ser async para poder usar .await
#[tokio::main]
async fn main() {
    // Ejecutando docker-compose en una nueva terminal
    if let Err(e) = ejecutar_comando() {
        println!("Error al ejecutar el comando en nueva terminal: {}", e);
    }

    // Pausar la ejecución durante 5 segundos para permitir que la terminal y el comando se inicien
    let wait_duration = Duration::from_secs(8); // Puedes ajustar este tiempo según sea necesario
    thread::sleep(wait_duration);

    // Verificando el endpoint /check/
    if let Err(e) = verificar_endpoint().await {
        println!("Error al verificar el endpoint: {}", e);
    }
}