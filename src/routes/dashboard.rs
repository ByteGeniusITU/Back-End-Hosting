use actix_web::{HttpResponse, Responder, web::Json};
use serde;
use std::process::{Command, Output};

#[derive(serde::Deserialize)]
pub struct ServerInfo {
    id: String,
}

#[derive(serde::Serialize, Debug)]
pub struct ServerStatus {
    cpu: u64,
    ram: f64,
}

pub async fn start(json: Json<ServerInfo>) -> impl Responder {
    let output = execute_command(&json.id, 1);

    if output.status.success() {
        HttpResponse::Ok()
            .body("Servidor abierto! Espere unos minutos hasta que se inicie por completo")
    } else {
        let stdout = String::from_utf8(output.stderr).unwrap();

        HttpResponse::InternalServerError().body(stdout)
    }
}

pub async fn stop(json: Json<ServerInfo>) -> impl Responder {
    let output = execute_command(&json.id, 0);

    if output.status.success() {
        HttpResponse::Ok().body("Servidor cerrado!")
    } else {
        let stdout = String::from_utf8(output.stderr).unwrap();

        HttpResponse::InternalServerError().body(stdout)
    }
}

fn execute_command(id: &str, replica: u8) -> Output {
    Command::new("kubectl")
        .args([
            "patch",
            "-n",
            "hosting",
            "deployment",
            &format!("minecraft-{}", id.to_lowercase()),
            "-p",
            &format!("{{\"spec\":{{\"replicas\":{}}}}}", replica),
        ])
        .output()
        .expect("Error al ejecutar el comando")
}

pub async fn status(json: Json<ServerInfo>) -> impl Responder {
    let output = Command::new("kubectl")
        .args([
            "get",
            "-n",
            "hosting",
            "pods",
            "-l",
            &format!("app=minecraft-{}", json.id),
            "-o",
            "jsonpath=\"{.items[0].metadata.name}\"",
        ])
        .output()
        .expect("Error al ejecutar el comando kubectl get pods");

    if !(output.status.success()) {
        let stdout = String::from_utf8(output.stderr).unwrap();

        return HttpResponse::InternalServerError().body(stdout);
    }

    let pod_name = String::from_utf8(output.stdout).unwrap();

    let pod_str = pod_name
        .strip_suffix("\"")
        .unwrap()
        .strip_prefix("\"")
        .unwrap();

    let output = Command::new("kubectl")
        .args(["top", "-n", "hosting", "pods", pod_str])
        .output()
        .expect("Error al ejecutar el comando kubectl top");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let usage_str = stdout.split("\n").collect::<Vec<&str>>()[1];

        let resouces_usage: Vec<&str> = usage_str.split_whitespace().collect();

        let mut status = ServerStatus {
            cpu: resouces_usage[1]
                .strip_suffix("m")
                .unwrap()
                .parse()
                .unwrap(),
            ram: resouces_usage[2]
                .strip_suffix("Mi")
                .unwrap()
                .parse()
                .unwrap(),
        };

        status.cpu /= 10;
        status.ram *= 1.04858_f64;

        HttpResponse::Ok().json(status)
    } else {
        let stdout = String::from_utf8(output.stderr).unwrap();

        HttpResponse::InternalServerError().body(stdout)
    }
}
