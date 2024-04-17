use actix_web::{web, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn receive_link(link: web::Json<String>) -> impl Responder {
    println!("Received link: {}", link);
    HttpResponse::Ok().json({"Received link".to_string()})
}
