#[macro_use]
extern crate lazy_static;

mod connection;
mod error;
mod user;
mod auth;

pub mod route;
pub use connection::pg_connection;