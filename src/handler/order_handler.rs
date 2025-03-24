use crate::model::Product;
use crate::model::{CreateOrder, Order};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_order(
    db_pool: web::Data<PgPool>,
    body: web::Json<CreateOrder>,
    path: web::Path<Uuid>,
) -> impl Responder {
    // Here basically what happened, The user is requested with user id into the params, and the (product id & product price) with the request body. then only product is checked is exists or not. if it exists, then order will created on the database.

    let user_id = &path.into_inner();
    let product_id = &body.product_id;
    let amount = &body.amount;

    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(&product_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match product {
        Ok(Some(product)) => {
            let order = sqlx::query_as::<_, Order>(
                "INSERT INTO orders (product_id, customer_id, amount, price) VALUES ($1,$2) RETURNING *",
            )
            .bind(product_id)
            .bind(user_id)
            .bind(amount)
            .bind(product.price)
            .fetch_optional(db_pool.get_ref())
            .await;

            match order {
                Ok(Some(order)) => HttpResponse::Ok().json(order),
                Ok(None) => HttpResponse::NotImplemented().body("No order has been created"),
                Err(err) => {
                    eprintln!("{err}");
                    HttpResponse::InternalServerError().body("Internal Server Error")
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().body("product not found"),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().body("Internal server Error")
        }
    }
}
