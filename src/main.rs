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
mod batch;
mod db;

use std::env;
use time::Duration;
use std::sync::Arc;
use anyhow::Result;
use scylla::Session;
use actix_cors::Cors;
use scylla::batch::Batch;
use scylla::query::Query;
use deadpool_postgres::Pool;
use error::Error as AppError;
use scylla::frame::value::ValueList;
use meilisearch_sdk::{client::Client};
use scylla::frame::value::BatchValues;
use scylla::{QueryResult, BatchResult};
use scylla::transport::errors::QueryError;
use scylla::prepared_statement::PreparedStatement;
use actix_web::{web, cookie, App as ActixApp, HttpServer};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};

#[derive(Clone)]
#[allow(dead_code)]
pub struct App {
    session: Arc<Session>,
    pool: Pool,
    indexer: Client,
}

impl App {
    fn new(session: Session, pool: Pool, indexer: Client) -> Self {
        Self {
            session: Arc::new(session),
            pool,
            indexer
        }
    }

    pub async fn query(&self, query: impl Into<Query>, values: impl ValueList) -> Result<QueryResult, QueryError>{
        self.session.query(query, values).await
    }

    pub async fn query_paged(&self, query: impl Into<Query>, values: impl ValueList, page: Vec<u8>) -> Result<QueryResult, QueryError>{
        let pagedata = Some(scylla::Bytes::from(page));
        self.session.query_paged(query, values, pagedata).await
    }

    pub async fn batch(&self, query: &Batch, values: impl BatchValues) -> Result<BatchResult, QueryError> {
        self.session.batch(query, values).await
    }

    pub async fn execute(&self, query: &PreparedStatement, values: impl ValueList) -> Result<QueryResult, QueryError> {
        self.session.execute(&query, values).await
    }
}

async fn start_server(app: App) -> Result<()> {
    let lp_host = env::var("LP_HOST").unwrap();
    let lp_port = env::var("LP_PORT").unwrap();
    let lp_port: u16 = lp_port.parse().unwrap();
    let pkey = env::var("PRIVATE_KEY").unwrap();
    let redis_uri = env::var("REDIS_URI").unwrap();

    let private_key = cookie::Key::from(pkey.as_bytes());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis_uri),
                    private_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::days(5))
                )
                .build()
            )
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind((lp_host, lp_port))?
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
    
    let session = db::get_scylla_connection().await;
    let pool = db::get_pg_connection().await;
    let indexer = db::get_indexer_connection().await;
    
    let app = App::new(session, pool, indexer);
    start_server(app).await.unwrap();
}