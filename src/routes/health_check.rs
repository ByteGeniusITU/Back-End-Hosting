use actix_web::{HttpResponse, Responder};

pub async fn healt_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
