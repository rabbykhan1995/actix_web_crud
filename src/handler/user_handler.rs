use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{CreateUser, User};

pub async fn get_user(db_pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user), // ✅ User found
        Ok(None) => HttpResponse::NotFound().body("User not found"), // ✅ No user found
        Err(err) => {
            eprintln!("Database error: {:?}", err); // ✅ Log the error
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
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
        "INSERT INTO users (id, name, email, password) 
         VALUES ($1, $2, $3, $4) 
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(email)
    .bind(password)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            eprintln!("user create failed");
            HttpResponse::NotImplemented().body("Registering Failed")
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err); // ✅ Log the error
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub async fn delete_user(db_pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match user {
        Ok(Some(_)) => {
            let delete = sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(id)
                .execute(db_pool.get_ref())
                .await;

            match delete {
                Ok(_) => HttpResponse::Ok().body("Delete Successfull"),
                Err(_) => HttpResponse::NotFound().body("User not found"),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(_) => HttpResponse::InternalServerError().body("internal server Error"),
    }
}
