use crate::query::UPDATE_BOOKS;
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::error::Error;
use super::model::UpdateRequest;

pub async fn update(
  app: web::Data<Pool>,
  payload: web::Json<UpdateRequest>,
  _: Session
) -> Result<HttpResponse, Error> {
  
  let conn = app.get().await?;
  conn.query(UPDATE_BOOKS, &[&payload.uid, &payload.title, &payload.body, &payload.metadata]).await?;

  Ok(HttpResponse::Ok().json(json!({
      "uid": &payload.uid,
      "title": &payload.title,
      "body": &payload.body,
      "metadata": &payload.metadata
  })))
}