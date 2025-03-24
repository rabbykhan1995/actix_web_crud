use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub company: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub discount: i32,
    pub stock: i32,
    pub price: i32,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(length(min = 2, message = "Company name must be at least 2 characters"))]
    pub company: String,

    #[validate(length(min = 5, message = "Title must be at least 5 characters"))]
    pub title: String,

    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    #[validate(length(min = 3, message = "Category must be at least 3 characters"))]
    pub category: String,

    #[validate(range(min = 0.0, message = "Discount must be positive"))]
    pub discount: f64,

    #[validate(range(min = 0, message = "Stock must be positive"))]
    pub stock: i32,

    #[validate(range(min = 0.0, message = "Price must be positive"))]
    pub price: f64,
}
