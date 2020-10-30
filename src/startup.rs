use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/", web::get().to(routes::greet))
            .route("/{name}", web::get().to(routes::greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
