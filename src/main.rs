use actix_web::{
    App, HttpServer,
    middleware::from_fn,
    web::{self, get, post, scope},
};

use dotenv::dotenv;
use std::env;

use sqlx::PgPool;

mod handler;
mod middleware;
mod model;
use handler::{
    admin_handler::get_admin,
    user_handler::{create_user, get_user},
};

use middleware::{admin_middleware, user_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("no database url found on env file");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect database");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/v1")
                // User Routes with Middleware
                .service(
                    scope("/user")
                        // .wrap(from_fn(my_middleware)) // Apply middleware only for "/user"
                        .route("/get-user/{id}", get().to(get_user))
                        .route("/create-user", post().to(create_user)), // Admin Routes (outside "/user")
                )
                .service(scope("/admin").route("/get-admin", get().to(get_admin))),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     name: String,
//     age: i16,
// }
// async fn another_user_handler(body: web::Json<User>) -> impl Responder {
//     println!("{:?}", body); // Fixed formatting issue
//     HttpResponse::Ok().body("Hello from another")
// }
