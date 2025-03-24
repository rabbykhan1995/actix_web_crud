use actix_web::{
    App, HttpServer,
    middleware::from_fn,
    web::{self, delete, get, post, scope},
};

use dotenv::dotenv;
use std::env;

use sqlx::PgPool;

mod handler;
mod middleware;
mod model;
use handler::{
    admin_handler::get_admin,
    order_handler::create_order,
    product_handler::{create_product, get_exact_product},
    user_handler::{create_user, delete_user, get_user},
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
                .service(
                    scope("/public")
                        .route("/product/{id}", get().to(get_exact_product))
                        .route("/product/create-product/{id}", post().to(create_product)),
                )
                // User Routes with Middleware
                .service(
                    scope("/user")
                        // .wrap(from_fn(my_middleware)) // Apply middleware only for "/user"
                        .route("/get-user/{id}", get().to(get_user))
                        .route("/create-user", post().to(create_user))
                        .route(
                            "/delete-user/{id}",
                            delete().to(delete_user), // Admin Routes (outside "/user")
                        ), // .service(scope("/admin").route("/get-admin", get().to(get_admin)))
                )
                .service(scope("/order").route("/create-order/{id}", post().to(create_order))),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
