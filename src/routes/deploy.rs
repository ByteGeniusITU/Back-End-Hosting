use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use serde;
use std::process::Command;

#[derive(serde::Deserialize)]
pub struct DeployInfo {
    id: String,
    cpu: u32,
    ram: u32,
}

pub async fn deploy_chart(deploy_info: Json<DeployInfo>) -> impl Responder {
    match Command::new("helm")
        .args([
            "install",
            &format!("minecraft-{}", deploy_info.id.to_lowercase()),
            "mc-charts/minecraft",
            "--set",
            &format!("minecraftServer.eula=true,minecraftServer.Difficulty=hard,minecraftServer.memory={}M,resources.limits.cpu={},resources.limits.memory={}M",
                (deploy_info.ram * 1024),
                deploy_info.cpu,
                (deploy_info.ram * 1024 + 200)
            ),
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
