use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::model::{CreateUser, ResponseJson, ResponseJsonWithResult, UpdateUser, User};

pub async fn get_user(db_pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user), // ✅ User found
        Ok(None) => HttpResponse::NotFound().json(ResponseJson {
            msg: "data not found".to_string(),
        }), // ✅ No user found
        Err(err) => {
            eprintln!("Database error: {:?}", err); // ✅ Log the error
            HttpResponse::InternalServerError().json(ResponseJson {
                msg: "Internal Server Error".to_string(),
            })
        }
    }
}

pub async fn create_user(
    db_pool: web::Data<PgPool>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    // First check the email which come from request body is valid or not.if valid then proceed, or return response.
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ResponseJsonWithResult {
            msg: "Invalid Input Found".to_string(),
            result: errors,
        });
    }

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
        Ok(Some(user)) => HttpResponse::Ok().json(ResponseJsonWithResult {
            msg: "user created successfull".to_string(),
            result: user,
        }),
        Ok(None) => {
            eprintln!("user create failed");
            HttpResponse::NotImplemented().json(ResponseJson {
                msg: "User not created For no reason".to_string(),
            })
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err); // ✅ Log the error
            HttpResponse::InternalServerError().json(ResponseJson {
                msg: "Internal Server Error".to_string(),
            })
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
                Ok(_) => HttpResponse::Ok().json(ResponseJson {
                    msg: "Delete Successfull".to_string(),
                }),
                Err(_) => HttpResponse::NotFound().json(ResponseJson {
                    msg: "Not found or delete failed".to_string(),
                }),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ResponseJson {
            msg: "User not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseJson {
            msg: "Internal Server Error".to_string(),
        }),
    }
}

pub async fn update_user(
    db_pool: web::Data<PgPool>,
    body: web::Json<UpdateUser>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id: Uuid = path.into_inner();

    let valid_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match valid_user {
        Ok(Some(user)) => {
            // Step 2: Merge new values with existing user
            let name = body.name.clone().unwrap_or(user.name);
            let email = body.email.clone().unwrap_or(user.email);
            let password = body.password.clone().unwrap_or(user.password);
            let user_types = body.user_types.clone().unwrap_or(user.user_types);

            let update = sqlx::query_as::<_, User>(
                "UPDATE users 
                   SET name = $1, email = $2, password = $3, user_types = $4
                   WHERE id = $5 RETURNING * ",
            )
            .bind(name)
            .bind(email)
            .bind(password)
            .bind(user_types)
            .bind(user.id)
            .fetch_one(db_pool.get_ref())
            .await;

            match update {
                Ok(updated_user) => HttpResponse::Ok().json(ResponseJsonWithResult {
                    msg: "updated successfully".to_string(),
                    result: updated_user,
                }),
                Err(_) => HttpResponse::InternalServerError().json(ResponseJson {
                    msg: "internal server error".to_string(),
                }),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ResponseJson {
            msg: "User Not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseJson {
            msg: "Internal Server Error".to_string(),
        }),
    }
}
