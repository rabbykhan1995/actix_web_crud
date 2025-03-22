use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

use crate::model::{CreateUser, User};

pub async fn get_user(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(_) => HttpResponse::InternalServerError().body("failed to get user"),
    }
}

pub async fn create_user(
    db_pool: web::Data<PgPool>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    let name = &body.name;
    let email = &body.email;
    let password = &body.password;

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&name)
    .bind(&email)
    .bind(&password)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}
