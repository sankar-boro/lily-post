use crate::query::DELETE_BOOKS;
use deadpool_postgres::Pool;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::error::Error;
use super::model::DeleteBookRequest;

pub async fn delete(
  app: web::Data<Pool>,
  payload: web::Json<DeleteBookRequest>,
  _: Session
) -> Result<HttpResponse, Error> {
  let conn = app.get().await?;
  conn.query(DELETE_BOOKS, &[&payload.uid]).await?;

  Ok(HttpResponse::Ok().body("Deleted book."))
}


