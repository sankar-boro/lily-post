#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
pub mod user;
pub mod error;

use mongodb::Client;
pub use error::Error as AppError;

pub async fn get_mongo_db_connection() -> Client {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client
}