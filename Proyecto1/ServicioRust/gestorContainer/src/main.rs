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
use std::io;
use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::Client;
use tokio;
use chrono::Utc;
use std::{thread, time::Duration};
use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::File;

// Definir estructura para deserializar la respuesta del endpoint /check/
#[derive(Deserialize)]
struct StatusResponse {
    status: String,
}

//********************************************************************************************************************************************
//                                                                                                            FUNCIONAMIENTO DE DOCKER COMPOSE
// Función para ejecutar el comando y obtener el ID del contenedor
fn obtener_id_contenedor() -> Result<String, io::Error> {
    // Ejecutar docker-compose en un proceso separado
    let status = Command::new("docker-compose")
        .args(&["up", "--build", "-d"]) // -d para modo detach
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al ejecutar docker-compose"));
    }

    // Obtener el ID del contenedor en ejecución
    let output = Command::new("docker")
        .arg("ps")
        .arg("-q")
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al obtener el ID del contenedor"));
    }

    // Convertir la salida a String y eliminar posibles caracteres de nueva línea
    let container_id = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .to_string();

    Ok(container_id)
}
// Función para detener el contenedor Docker dado su ID
fn detener_contenedor(container_id: &str) -> Result<(), io::Error> {
    // Ejecutar el comando `docker stop` con el ID del contenedor
    let status = Command::new("docker")
        .arg("stop")
        .arg(container_id)
        .status()?;

    // Verificar si el comando se ejecutó correctamente
    if status.success() {
        println!("Contenedor detenido exitosamente: {}", container_id);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Error al detener el contenedor"))
    }
}

// Función para abrir una nueva terminal
fn abrir_terminal() -> std::io::Result<()> {
    Command::new("gnome-terminal")
        .arg("--")
        .arg("bash")
        .arg("-c")
        .arg("cd /home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs && docker-compose up --build; exec bash")
        .spawn()?;  // Usamos spawn para no bloquear la ejecución
    Ok(())
}
//********************************************************************************************************************************************
//                                                                                                                 FUNCIONAMIENTO DE ENDPOINTS
// Función asíncrona para verificar el endpoint /check/
async fn verificar_endpoint() -> Result<(), Box<dyn Error>> {
    // Crear cliente HTTP
    let client = Client::new();

    // Dirección del contenedor de Python
    let python_container_url = "http://0.0.0.0:8000";

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

//********************************************************************************************************************************************
//                                                                                              FUNCIONAMIENTO DE LA LECTURA SYSINFO_201901103

#[derive(Deserialize)]
struct RamInfo {
    total: u64,
    libre: u64,
    uso: u64,
}

#[derive(Deserialize)]
struct Container {
    pid: String,
    nombre: String,
    idContainer: String,
    Vsz: u64,
    Rss: u64,
    #[serde(rename = "memoria_usage", deserialize_with = "deserialize_percentage")]
    memoria_usage: f64, // Convertimos el porcentaje a un valor decimal
    #[serde(rename = "cpu_usage", deserialize_with = "deserialize_percentage")]
    cpu_usage: f64, // Convertimos el porcentaje a un valor decimal
}

#[derive(Deserialize)]
struct ContainersInfo {
    ram: RamInfo,
    containers: Vec<Container>,
}

// Función para deserializar porcentajes del tipo "7%" a f64
fn deserialize_percentage<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let percentage = s.trim_end_matches('%').parse::<f64>().map_err(serde::de::Error::custom)?;
    Ok(percentage)
}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<ContainersInfo, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // Eliminar cualquier coma final antes de deserializar el JSON
    let cleaned_contents = contents.replace(",\n  ]", "\n  ]");

    let containers_info: ContainersInfo = serde_json::from_str(&cleaned_contents)?;
    Ok(containers_info)
}

fn identify_consumers(containers: &[Container]) {
    let high_cpu_threshold = 10.0; // Ajustado a un valor más bajo para fines de demostración
    let low_cpu_threshold = 5.0;  // Ajustado a un valor más bajo para fines de demostración

    let high_consumers: Vec<&Container> = containers
        .iter()
        .filter(|&c| c.cpu_usage > high_cpu_threshold)
        .collect();

    let low_consumers: Vec<&Container> = containers
        .iter()
        .filter(|&c| c.cpu_usage < low_cpu_threshold)
        .collect();

    println!("High CPU Consumers:");
    for container in high_consumers {
        println!(
            "ID: {}, Nombre: {}, PID: {}, Vsz: {}Kbs, Rss: {}Kbs, CPU Usage: {}%, RAM Usage: {}%",
            container.idContainer, container.nombre, container.pid, container.Vsz, container.Rss, container.cpu_usage, container.memoria_usage
        );
    }

    println!("\nLow CPU Consumers:");
    for container in low_consumers {
        println!(
            "ID: {}, Nombre: {}, PID: {}, Vsz: {}Kbs, Rss: {}Kbs, CPU Usage: {}%, RAM Usage: {}%",
            container.idContainer, container.nombre, container.pid, container.Vsz, container.Rss, container.cpu_usage, container.memoria_usage
        );
    }
}


// Nueva función async que contiene la lógica de main
async fn process_containers() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/proc/sysinfo_201901103";
    let containers_info = read_json_file(path)?;

    println!(
        "RAM Total: {}MB, RAM Libre: {}MB, RAM Uso: {}MB",
        containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );

    identify_consumers(&containers_info.containers);

    Ok(())
}
//********************************************************************************************************************************************
//                                                                                                                 FUNCIONAMIENTO DEL MAIN
// El main debe ser async para poder usar .await
#[tokio::main]
async fn main() {
    // Ejecutando docker-compose en una nueva terminal
    if let Err(e) = abrir_terminal() {
        println!("Error al abrir la terminal: {}", e);
    }

    // Pausar la ejecución durante 8 segundos para permitir que la terminal y el comando se inicien
    let wait_duration = Duration::from_secs(8); // Puedes ajustar este tiempo según sea necesario
    thread::sleep(wait_duration);

    // Obtener el ID del contenedor
    let container_id = match obtener_id_contenedor() {
        Ok(id) => id,
        Err(e) => {
            println!("Error al obtener el ID del contenedor: {}", e);
            return;
        }
    };

    println!("ID del contenedor: {}", container_id);

    /*
            De acá se debe agregar para que sea un ciclo de 10 segundos y se vuelva a repetir iniciando desde este punto
    */
    /*
    // Leyendo el contenido de /proc/sysinfo_201901103
    if let Err(e) = process_containers().await {
        eprintln!("Error: {}", e);
    }*/

    // Verificando el endpoint /check/
    if let Err(e) = verificar_endpoint().await {
        println!("Error al verificar el endpoint: {}", e);
    }

    /*
     // Llamar a la función para detener el contenedor
     if let Err(e) = detener_contenedor(&container_id) {
        println!("Error al detener el contenedor: {}", e);
    }*/


}