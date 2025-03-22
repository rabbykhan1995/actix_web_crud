use actix_web::{Responder, HttpResponse};

pub async fn get_admin() -> impl Responder {
    HttpResponse::Ok().body("hello from admin")
}