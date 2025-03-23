// Declare the user_model module
pub mod product_model;
pub mod user_model;

// Re-export User and CreateUser from user_model for easier access
pub use product_model::{CreateProduct, Product};
pub use user_model::{CreateUser, User};
