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

use time::Duration;
use std::sync::Arc;
use anyhow::Result;
use error::Error as AppError;
// use actix_redis::RedisSession;
use scylla::batch::Batch;
use scylla::{
    Session, 
    SessionBuilder
};
use actix_web::{web, cookie, App as ActixApp, HttpServer};
use actix_cors::Cors;

use scylla::{QueryResult, BatchResult};
use scylla::query::Query;
use scylla::frame::value::ValueList;
use scylla::frame::value::BatchValues;
use scylla::transport::errors::QueryError;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::env;
use meilisearch_sdk::{
    client::Client
};
use scylla::prepared_statement::PreparedStatement;
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
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let private_key = cookie::Key::from("authUser".as_bytes());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                // RedisSession::new("127.0.0.1:6379", &[0; 32])
                // .cookie_name("lily-session")
                // .cookie_http_only(true)
                // .ttl(86400)
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
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
    .bind(format!("{}:{}", host, port))?
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
    let mut cfg = Config::new();
    cfg.dbname = Some("sankar".to_string());
    cfg.user = Some("sankar".to_string());
    cfg.password = Some("sankar".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let session = SessionBuilder::new().known_node(uri).build().await.unwrap();
    let indexer = Client::new("http://localhost:7700", Some("authUser"));
    let app = App::new(session, pool, indexer);
    start_server(app).await.unwrap();
}