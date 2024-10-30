use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::io::Result;
use std::thread;
use tonic::transport::Channel;
use tokio::runtime::Runtime;

#[derive(Deserialize)]
struct RequestBody {
    student: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

mod student_service {
    tonic::include_proto!("student_service"); // Ajusta el nombre de paquete según tu archivo .proto
}

async fn send_to_server(body: RequestBody, address: &'static str) {
    let channel = Channel::from_shared(address.to_string()).unwrap().connect().await.unwrap();
    let mut client = student_service::student_service_client::StudentServiceClient::new(channel);

    let request = tonic::Request::new(student_service::StudentRequest {
        student: body.student,
        age: body.age,
        faculty: body.faculty,
        discipline: body.discipline,
    });

    match client.send_student(request).await {
        Ok(_) => println!("Student sent successfully"),
        Err(e) => eprintln!("Failed to send student: {:?}", e),
    }
}

#[post("/submitIngenieria")]
async fn submit_agronomia(body: web::Json<RequestBody>) -> impl Responder {
    println!(
        "Student: {}, Age: {}, Faculty: {}, Discipline: {}",
        body.student, body.age, body.faculty, body.discipline
    );

    // Determinar el servidor gRPC según la disciplina
    let address = match body.discipline {
        1 => "http://localhost:8082", // Natación
        2 => "http://localhost:8083", // Atletismo
        3 => "http://localhost:8084", // Boxeo
        _ => return HttpResponse::BadRequest().body("Invalid discipline"),
    };

    let body_clone = body.into_inner();

    // Lanzar un thread para enviar la solicitud gRPC
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(send_to_server(body_clone, address));
    });

    let response = format!(
        "Received student: {}, age: {}, faculty: {}, discipline: {}",
        body_clone.student, body_clone.age, body_clone.faculty, body_clone.discipline
    );

    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Server running on port 8081");
    HttpServer::new(|| App::new().service(submit_agronomia))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
