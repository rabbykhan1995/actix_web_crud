use actix_web::{HttpResponse, Responder};

use crate::model::ResponseJson;

pub async fn get_admin() -> impl Responder {
    HttpResponse::Ok().json(ResponseJson {
        msg: "This is admin Route",
    })
}
