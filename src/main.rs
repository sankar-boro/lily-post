#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
mod route;
mod user;
mod helpers;
mod middleware;
mod utils;
mod error;
mod query;
mod book;
mod blog;
mod auth;
mod booknode;
mod blognode;
mod settings;

use std::sync::Arc;
use anyhow::Result;
use error::Error as AppError;
use actix_redis::RedisSession;
use scylla::batch::Batch;
use scylla::{
    Session, 
    SessionBuilder
};
use actix_web::{App as ActixApp, HttpServer};
use actix_web::web::{
    self,
};
use actix_cors::Cors;

use scylla::{QueryResult, BatchResult};
use scylla::query::Query;
use scylla::frame::value::ValueList;
use scylla::frame::value::BatchValues;
use scylla::transport::errors::QueryError;

#[derive(Clone)]
pub struct App {
    session: Arc<Session>,
}

impl App {
    fn new(session: Session) -> Self {
        Self {
            session: Arc::new(session),
        }
    }

    pub async fn query(&self, query: impl Into<Query>, values: impl ValueList) -> Result<QueryResult, QueryError>{
        self.session.query(query, values).await
    }

    pub async fn query_paged(&self, query: impl Into<Query>, values: impl ValueList, page: Vec<u8>) -> Result<QueryResult, QueryError>{
        let pagedata = Some(scylla::Bytes::from(page));
        self.session.query_paged(query, values, pagedata).await
    }

    pub async fn batch(&self, query: &Batch, values: impl BatchValues) -> Result<BatchResult, QueryError>{
        self.session.batch(query, values).await
    }
}

async fn start_server(app: App) -> Result<()> {

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                RedisSession::new("127.0.0.1:6379", &[0; 32])
                .cookie_name("lily-session")
                .cookie_http_only(true)
                .ttl(86400)
            )
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind("127.0.0.1:7501")?
    .run()
    .await?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let uri = "127.0.0.1:9042";
    let session = SessionBuilder::new().known_node(uri).build().await.unwrap();
    let app = App::new(session);
    start_server(app).await.unwrap();
}