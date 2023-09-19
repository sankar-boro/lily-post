use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::error::Error;
use super::model::UpdateRequest;

pub static UPDATE_BOOK: &str = "UPDATE book SET title=$1, body=$2, metadata=$3 WHERE uid=$4";
pub static UPDATE_BOOKNODE: &str = "UPDATE booknode SET title=$1, body=$2, metadata=$3 WHERE uid=$4";
pub static UPDATE_TITLE: &str = "UPDATE title SET title=$1 WHERE uid=$2";

pub async fn update(
  app: web::Data<Pool>,
  payload: web::Json<UpdateRequest>,
  _: Session
) -> Result<HttpResponse, Error> {
  
  let conn = app.get().await?;
  if payload.identity == 101 {
    conn.query(UPDATE_BOOK, &[&payload.title, &payload.body, &payload.metadata, &payload.docid]).await?;
  }
  conn.query(UPDATE_BOOKNODE, &[&payload.title, &payload.body, &payload.metadata, &payload.uid]).await?;
  conn.query(UPDATE_TITLE, &[&payload.title, &payload.uid]).await?;


  Ok(HttpResponse::Ok().json(json!({
      "uid": &payload.uid,
      "title": &payload.title,
      "body": &payload.body,
      "metadata": &payload.metadata
  })))
}