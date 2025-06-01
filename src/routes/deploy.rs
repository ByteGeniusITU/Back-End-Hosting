use actix_web::{HttpResponse, Responder};
use std::process::Command;

pub async fn deploy_chart() -> impl Responder {
    match Command::new("helm")
        .args([
            "install",
            "minecraft",
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
