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
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

// Definir estructura para deserializar la respuesta del endpoint /check/
#[derive(Deserialize)]
struct StatusResponse {
    status: String,
}

// Función para ejecutar el comando y obtener el ID del contenedor
fn obtener_id_contenedor() -> Result<String, io::Error> {
    let status = Command::new("docker-compose")
        .args(&["up", "--build", "-d"])
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al ejecutar docker-compose"));
    }

    let output = Command::new("docker")
        .arg("ps")
        .arg("-q")
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al obtener el ID del contenedor"));
    }

    let container_id = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .to_string();

    Ok(container_id)
}

// Función para detener el contenedor Docker dado su ID
fn detener_contenedor(container_id: &str) -> Result<(), io::Error> {
    let status = Command::new("docker")
        .arg("stop")
        .arg(container_id)
        .status()?;

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
        .spawn()?;
    Ok(())
}

// Función asíncrona para verificar el endpoint /check/
async fn verificar_endpoint() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let python_container_url = "http://0.0.0.0:8000";

    let response = client
        .get(&format!("{}/check/", python_container_url))
        .send()
        .await?;

    if response.status().is_success() {
        let status_response: StatusResponse = response.json().await?;
        println!("Respuesta del endpoint /check/: {}", status_response.status);
    } else {
        println!("Error en la petición al endpoint /check/: {:?}", response.status());
    }

    Ok(())
}

// Estructura para deserializar la información de la RAM
#[derive(Deserialize)]
struct RamInfo {
    total: u64,
    libre: u64,
    uso: u64,
}

// Estructura para deserializar la información de los contenedores
#[derive(Deserialize)]
struct Container {
    pid: String,
    nombre: String,
    cmdline: String,
    vsz: u64,
    rss: u64,
    memoria_usage: f64,
    cpu_usage: f64,
}

// Estructura general que contiene la información de la RAM y los contenedores
#[derive(Deserialize)]
struct ContainersInfo {
    ram: RamInfo,
    containers: Vec<Container>,
}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<ContainersInfo, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("Contenido del archivo JSON:\n{}", contents);

    let containers_info: ContainersInfo = serde_json::from_str(&contents)?;
    Ok(containers_info)
}

// Función para eliminar un contenedor
fn kill_container(id: &str) {
    let output = Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Matando contenedor con id: {}", id);
    println!("Output: {:?}", output);
}
/*
// Función para identificar y eliminar contenedores
fn identify_consumers(containers_info: ContainersInfo, log_container_pid: &str) {
    let mut containers = containers_info.containers;

    // Ordena los contenedores en base a cpu_usage, memoria_usage, vsz y rss
    containers.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(Ordering::Equal)
            .then_with(|| b.memoria_usage.partial_cmp(&a.memoria_usage).unwrap_or(Ordering::Equal))
            .then_with(|| b.vsz.partial_cmp(&a.vsz).unwrap_or(Ordering::Equal))
            .then_with(|| b.rss.partial_cmp(&a.rss).unwrap_or(Ordering::Equal))
    });

    let mut high_consumers = containers.split_off(containers.len().saturating_sub(3));
    let mut low_consumers = containers.split_off(2);

    let log_container_pid = log_container_pid.to_string();

    let mut containers_to_remove: Vec<String> = Vec::new();

    // En la lista de alto consumo, eliminamos todos los contenedores excepto los 2 primeros
    if high_consumers.len() > 2 {
        for container in high_consumers.iter().skip(2) {
            if container.pid != log_container_pid {
                containers_to_remove.push(container.pid.clone());
            }
        }
    }

    // En la lista de bajo consumo, eliminamos todos los contenedores excepto los 3 últimos
    if low_consumers.len() > 3 {
        for container in low_consumers.iter().take(low_consumers.len() - 3) {
            if container.pid != log_container_pid {
                containers_to_remove.push(container.pid.clone());
            }
        }
    }

    // Ejecutar la eliminación en paralelo usando hilos
    let containers_to_remove = Arc::new(Mutex::new(containers_to_remove));
    let mut handles = vec![];

    for _ in 0..containers_to_remove.lock().unwrap().len() {
        let containers_to_remove = Arc::clone(&containers_to_remove);
        let handle = thread::spawn(move || {
            let pids = containers_to_remove.lock().unwrap();
            for pid in pids.iter() {
                kill_container(pid);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Imprimir la información de la RAM
    println!(
        "RAM Total: {} Kb, RAM Libre: {} Kb, RAM Uso: {} Kb",
        containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );

    // Imprimir los contenedores restantes
    println!("Contenedores restantes:");
    for container in containers {
        println!(
            "cmdline: {}, Nombre: {}, PID: {}, Vsz: {} Kb, Rss: {} Kb, CPU Usage: {:.2}%, RAM Usage: {:.2}%",
            container.cmdline, container.nombre, container.pid, container.vsz, container.rss, container.cpu_usage * 100.0, container.memoria_usage * 100.0
        );
    }
    println!("Antes de los logs");
    // Generar logs
    let log_time = Utc::now().to_rfc3339();
    let log_message = format!(
        "[{}] Información de la RAM: Total: {} Kb, Libre: {} Kb, Uso: {} Kb",
        log_time, containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );
    println!("{}", log_message);

    // Enviar petición HTTP al contenedor de logs
    let client = Client::new();
    let log_container_url = "http://0.0.0.0:8000"; // Dirección del contenedor administrador de logs

    tokio::spawn(async move {
        if let Err(e) = client
            .post(&format!("{}/log/", log_container_url))
            .json(&log_message)
            .send()
            .await
        {
            eprintln!("Error al enviar log al contenedor de logs: {}", e);
        }
    });
}*/
fn identify_consumers(containers_info: ContainersInfo, log_container_pid: &str) {
    let mut containers = containers_info.containers;

    // Ordenar los contenedores por CPU, memoria, vsz y rss
    containers.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(Ordering::Equal)
            .then_with(|| b.memoria_usage.partial_cmp(&a.memoria_usage).unwrap_or(Ordering::Equal))
            .then_with(|| b.vsz.partial_cmp(&a.vsz).unwrap_or(Ordering::Equal))
            .then_with(|| b.rss.partial_cmp(&a.rss).unwrap_or(Ordering::Equal))
    });

    // Información de la RAM
    println!("--- Información de la RAM ---");
    println!(
        "RAM Total: {} Kb, RAM Libre: {} Kb, RAM Uso: {} Kb",
        containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );
    
    // Verificar si hay al menos 2 contenedores de alto consumo
    let high_consumers = if containers.len() > 2 {
        containers.split_off(containers.len() - 2)
    } else {
        Vec::new()
    };

    // Verificar si hay al menos 3 contenedores de bajo consumo
    let low_consumers = if containers.len() > 3 {
        containers.split_off(containers.len() - 3)
    } else {
        Vec::new()
    };

    let log_container_pid = log_container_pid.to_string();
    let mut containers_to_remove: Vec<String> = Vec::new();

    // Imprimir los 2 contenedores de alto consumo
    println!("--- Contenedores de alto consumo (Top 2) ---");
    for container in &high_consumers {
        println!(
            "cmdline: {}, Nombre: {}, PID: {}, Vsz: {} Kb, Rss: {} Kb, CPU Usage: {:.2}%, RAM Usage: {:.2}%",
            container.cmdline, container.nombre, container.pid, container.vsz, container.rss, container.cpu_usage * 100.0, container.memoria_usage * 100.0
        );
    }

    // Imprimir los 3 contenedores de bajo consumo
    println!("--- Contenedores de bajo consumo (Top 3) ---");
    for container in &low_consumers {
        println!(
            "cmdline: {}, Nombre: {}, PID: {}, Vsz: {} Kb, Rss: {} Kb, CPU Usage: {:.2}%, RAM Usage: {:.2}%",
            container.cmdline, container.nombre, container.pid, container.vsz, container.rss, container.cpu_usage * 100.0, container.memoria_usage * 100.0
        );
    }

    // Determinar qué contenedores eliminar (evitar eliminar el contenedor de logs)
    for container in high_consumers.iter().skip(2) {
        if container.pid != log_container_pid {
            containers_to_remove.push(container.pid.clone());
        }
    }

    for container in low_consumers.iter().take(low_consumers.len().saturating_sub(3)) {
        if container.pid != log_container_pid {
            containers_to_remove.push(container.pid.clone());
        }
    }

    // Imprimir los contenedores eliminados
    println!("--- Contenedores eliminados ---");
    if containers_to_remove.is_empty() {
        println!("No se eliminaron contenedores.");
    } else {
        for container_id in &containers_to_remove {
            println!("Contenedor eliminado con PID: {}", container_id);
        }
    }

    // Eliminar los contenedores
    let containers_to_remove = Arc::new(Mutex::new(containers_to_remove));
    let mut handles = vec![];

    for _ in 0..containers_to_remove.lock().unwrap().len() {
        let containers_to_remove = Arc::clone(&containers_to_remove);
        let handle = thread::spawn(move || {
            let pids = containers_to_remove.lock().unwrap();
            for pid in pids.iter() {
                kill_container(pid);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Generar logs
    println!("--- Logs ---");
    let log_time = Utc::now().to_rfc3339();
    let log_message = format!(
        "[{}] Información de la RAM: Total: {} Kb, Libre: {} Kb, Uso: {} Kb",
        log_time, containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );
    println!("{}", log_message);

    // Enviar los logs al contenedor de logs
    let client = Client::new();
    let log_container_url = "http://0.0.0.0:8000"; // Dirección del contenedor administrador de logs

    tokio::spawn(async move {
        if let Err(e) = client
            .post(&format!("{}/log/", log_container_url))
            .json(&log_message)
            .send()
            .await
        {
            eprintln!("Error al enviar log al contenedor de logs: {}", e);
        }
    });
}


// Función principal que procesa los contenedores
async fn process_containers(log_container_pid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "/proc/sysinfo_201901103";  // Ruta al archivo JSON
    let containers_info = read_json_file(path)?;  // Lee el JSON

    // Imprime la información de la RAM
    println!(
        "RAM Total: {} Kb, RAM Libre: {} Kb, RAM Uso: {} Kb",
        containers_info.ram.total, containers_info.ram.libre, containers_info.ram.uso
    );

    identify_consumers(containers_info, log_container_pid);
    Ok(())
}
// Función para ejecutar el comando y obtener el ID y PID del contenedor
fn obtener_pid_contenedor() -> Result<(String, String), io::Error> {
    let status = Command::new("docker-compose")
        .args(&["up", "--build", "-d"])
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al ejecutar docker-compose"));
    }

    // Obtiene el ID del contenedor
    let output_id = Command::new("docker")
        .arg("ps")
        .arg("-q")
        .current_dir("/home/pjd/Documentos/sopes/SO1_2S2024_201901103/Proyecto1/ContainerAdLogs")
        .output()?;

    if !output_id.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al obtener el ID del contenedor"));
    }

    let container_id = String::from_utf8(output_id.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .to_string();

    // Obtiene el PID del contenedor usando el ID
    let output_pid = Command::new("docker")
        .arg("inspect")
        .arg("--format='{{.State.Pid}}'")
        .arg(&container_id)
        .output()?;

    if !output_pid.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Error al obtener el PID del contenedor"));
    }

    let container_pid = String::from_utf8(output_pid.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .replace("'", "")
        .to_string();

    Ok((container_id, container_pid))
}

// Función principal del programa
#[tokio::main]
async fn main() {
    if let Err(e) = abrir_terminal() {
        println!("Error al abrir la terminal: {}", e);
    }

    // Obtener ID y PID del contenedor
    let (container_id, log_container_pid) = match obtener_pid_contenedor() {
        Ok((id, pid)) => (id, pid),
        Err(e) => {
            println!("Error al obtener el ID o PID del contenedor: {}", e);
            return;
        }
    };

    println!("ID del contenedor: {}", container_id);
    println!("PID del contenedor de logs: {}", log_container_pid);
    let pid_as_string = log_container_pid.to_string();
    let wait_duration = Duration::from_secs(10);
        thread::sleep(wait_duration);
    //let log_container_pid = ""; // Debe ser el PID del contenedor de logs, necesitarás implementarlo

    //loop {
        if let Err(e) = process_containers(&pid_as_string).await {
            eprintln!("Error: {}", e);
        }

        if let Err(e) = verificar_endpoint().await {
            println!("Error al verificar el endpoint: {}", e);
        }
        println!("Antes del endpoint");
        let wait_duration = Duration::from_secs(10);
        thread::sleep(wait_duration);

    //}

    if let Err(e) = detener_contenedor(&container_id) {
        println!("Error al detener el contenedor: {}", e);
    }
}
