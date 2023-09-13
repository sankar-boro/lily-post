#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

mod user;
mod auth;
mod book;
mod blog;
mod query;
mod error;
mod booknode;
mod blognode;
mod connection;

pub mod route;
pub use connection::pg_connection;