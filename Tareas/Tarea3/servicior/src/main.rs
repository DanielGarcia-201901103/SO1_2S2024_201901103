use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Deserialize)]
struct Container {
    id: String,
    cpu_usage: f64,
    ram_usage: u64,
}

#[derive(Deserialize)]
struct ContainersInfo {
    containers: Vec<Container>,
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
            "ID: {}, CPU Usage: {}%, RAM Usage: {}MB",
            container.id, container.cpu_usage, container.ram_usage
        );
    }

    println!("\nLow CPU Consumers:");
    for container in low_consumers {
        println!(
            "ID: {}, CPU Usage: {}%, RAM Usage: {}MB",
            container.id, container.cpu_usage, container.ram_usage
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/proc/sysinfo_201901103";
    let containers_info = read_json_file(path)?;

    identify_consumers(&containers_info.containers);

    Ok(())
}
