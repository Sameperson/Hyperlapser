use actix_web::{App, HttpServer};
use crate::routes::general_routes;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(general_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
