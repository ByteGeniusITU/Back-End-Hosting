use actix_web::{HttpResponse, Responder, web::Json};
use serde;
use std::process::{Command, Output};

#[derive(serde::Deserialize)]
pub struct ServerInfo {
    username: String,
}

pub async fn start(json: Json<ServerInfo>) -> impl Responder {
    let output = execute_command(&json.username, 1);

    if output.status.success() {
        HttpResponse::Ok()
            .body("Servidor abierto! Espere unos minutos hasta que se inicie por completo")
    } else {
        let stdout = String::from_utf8(output.stderr).unwrap();

        HttpResponse::InternalServerError().body(stdout)
    }
}

pub async fn stop(json: Json<ServerInfo>) -> impl Responder {
    let output = execute_command(&json.username, 0);

    if output.status.success() {
        HttpResponse::Ok().body("Servidor cerrado!")
    } else {
        let stdout = String::from_utf8(output.stderr).unwrap();

        HttpResponse::InternalServerError().body(stdout)
    }
}

fn execute_command(username: &str, replica: u8) -> Output {
    Command::new("kubectl")
        .args([
            "patch",
            "deployment",
            &format!("minecraft-{}", username.to_lowercase()),
            "-p",
            &format!("{{\"spec\":{{\"replicas\":{}}}}}", replica),
        ])
        .output()
        .expect("Error al ejecutar el comando")
}
