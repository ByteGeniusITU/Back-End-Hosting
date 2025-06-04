use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use serde;
use std::process::Command;

#[derive(serde::Deserialize)]
pub struct DeployInfo {
    username: String,
    cpu: u8,
    ram: u8,
}

pub async fn deploy_chart(deploy_info: Json<DeployInfo>) -> impl Responder {
    match Command::new("helm")
        .args([
            "install",
            &format!("minecraft-{}", deploy_info.username.to_lowercase()),
            "mc-charts/minecraft",
            "--set",
            "minecraftServer.eula=true,minecraftServer.Difficulty=hard",
        ])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8(output.stdout).unwrap();

                HttpResponse::Ok().body(stdout)
            } else {
                let stderr = String::from_utf8(output.stderr).unwrap();

                HttpResponse::BadRequest().body(stderr)
            }
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}
