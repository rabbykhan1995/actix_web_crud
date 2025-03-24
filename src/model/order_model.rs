use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub amount: i32,
    pub price: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub status: String,
    pub amount: i32,
    pub price: i32,
}
