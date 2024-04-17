use actix_web::web;
use crate::handlers::{index, receive_link};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/submit", web::post().to(receive_link));
}
