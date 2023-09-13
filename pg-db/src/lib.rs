#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

mod connection;
mod error;
mod user;
mod auth;
mod book;
mod booknode;
mod query;

pub mod route;
pub use connection::pg_connection;