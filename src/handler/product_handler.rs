use crate::model::{CreateProduct, Product};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub async fn get_exact_product(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let product_id = path.into_inner();

    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(product_id)
        .fetch_optional(db_pool.get_ref())
        .await;

    match product {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("No product found"),
        Err(err) => {
            eprintln!("Internale server Error {}", err);
            HttpResponse::InternalServerError().body("internal server error")
        }
    }
}

pub async fn create_product(
    db_pool: web::Data<PgPool>,
    body: web::Json<CreateProduct>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    println!("Received request: {:?}", body);
    println!("User ID: {:?}", user_id);

    if let Err(error) = body.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products 
         (name, company, title, description, category, discount, stock, price, created_by) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
         RETURNING *",
    )
    .bind(&body.name)
    .bind(&body.company)
    .bind(&body.title)
    .bind(&body.description)
    .bind(&body.category)
    .bind(&body.discount)
    .bind(&body.stock)
    .bind(&body.price)
    .bind(&user_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match product {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("Not created"),
        Err(err) => {
            eprintln!("{err}");
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
