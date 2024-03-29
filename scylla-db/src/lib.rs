#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
pub mod route;
pub mod user;
pub mod helpers;
pub mod utils;
pub mod error;
pub mod query;
pub mod book;
pub mod blog;
pub mod auth;
pub mod booknode;
pub mod blognode;
pub mod settings;
pub mod batch;
pub mod db;
pub mod builder;
pub mod client;
pub use builder::Connections;
pub use error::Error as AppError;

pub async fn get_scylla_db_connection() -> Connections {

  let session = db::get_scylla_connection().await;
  let pool = db::get_pg_connection().await;
  
  let conn = Connections::new(session, pool);
  conn
}
// use std::env;
// use anyhow::Result;
// use actix_cors::Cors;
// use actix_web::{web, cookie, App as ActixApp, HttpServer};
// use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};

// async fn start_server(app: Connections) -> Result<()> {
//     let lp_host = env::var("LP_HOST").unwrap();
//     let lp_port = env::var("LP_PORT").unwrap();
//     let lp_port: u16 = lp_port.parse().unwrap();
//     let pkey = env::var("PRIVATE_KEY").unwrap();
//     let redis_uri = env::var("REDIS_URI").unwrap();

//     let private_key = cookie::Key::from(pkey.as_bytes());

//     HttpServer::new(move || {
//         let cors = Cors::default()
//               .allow_any_origin()
//               .allow_any_method()
//               .allow_any_header()
//               .supports_credentials();

//         ActixApp::new()
//             .wrap(cors)
//             .wrap(
//                 SessionMiddleware::builder(
//                     RedisActorSessionStore::new(&redis_uri),
//                     private_key.clone(),
//                 )
//                 .session_lifecycle(
//                     PersistentSession::default()
//                         .session_ttl(Duration::days(5))
//                 )
//                 .build()
//             )
//             .app_data(web::Data::new(app.clone()))
//             .configure(route::routes)
//     })
//     .bind((lp_host, lp_port))?
//     .run()
//     .await?;
//     Ok(())
// }

// #[actix_web::main]
// async fn main() {
//     std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
//     std::env::set_var("RUST_LOG", "info");
//     std::env::set_var("RUST_BACKTRACE", "1");
//     env_logger::init();
    
//     let session = db::get_scylla_connection().await;
//     let pool = db::get_pg_connection().await;
    
//     let app = Connections::new(session, pool);
//     start_server(app).await.unwrap();
// }