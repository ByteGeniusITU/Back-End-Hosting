use crate::routes::*;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .route("/health_check", web::get().to(healt_check))
            .route("/server", web::post().to(deploy_chart))
            .route("/dashboard/start", web::post().to(start))
            .route("/dashboard/stop", web::post().to(stop))
            .route("/dashboard/status", web::post().to(status))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
