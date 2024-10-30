use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::io::Result;

#[derive(Deserialize)]
struct RequestBody {
    student: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

#[post("/submitIngenieria")]
async fn submit_agronomia(body: web::Json<RequestBody>) -> impl Responder {
    println!(
        "Student: {}, Age: {}, Faculty: {}, Discipline: {}",
        body.student, body.age, body.faculty, body.discipline
    );

    let response = format!(
        "Received student: {}, age: {}, faculty: {}, discipline: {}",
        body.student, body.age, body.faculty, body.discipline
    );

    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Server running on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(submit_agronomia)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
