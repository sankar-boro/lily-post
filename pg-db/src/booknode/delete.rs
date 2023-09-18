use deadpool_postgres::Pool;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::error::Error;
use super::model::DeleteNode;

pub static UPDATE_NODE: &str = "UPDATE booknode SET parentid=$1 WHERE uid=$2";
pub static UPDATE_TITLE: &str = "UPDATE title SET parentid=$1 WHERE uid=$2";

pub async fn delete(
  app: web::Data<Pool>,
  payload: web::Json<DeleteNode>,
  _: Session
) -> Result<HttpResponse, Error> {
  let conn = app.get().await?;
  let mut ids = String::from("");
  let mut _ids = payload.nodes.iter();
  if let Some(firstid) = _ids.next() {
    ids.push_str(&firstid.to_string());
  }
  while let Some(id) = _ids.next() {
    ids.push(',');
    ids.push_str(&id.to_string());
  }
  let mut delete_title = "DELETE FROM title where uid IN (".to_string();
  delete_title.push_str(&ids);
  delete_title.push(')');

  let mut delete_node = "DELETE FROM booknode where uid IN (".to_string();
  delete_node.push_str(&ids);
  delete_node.push(')');

  conn.query(&delete_title, &[]).await?;
  conn.query(&delete_node, &[]).await?;

  if let Some(update_data) = &payload.update {
    conn.query(UPDATE_NODE, &[&update_data.tuid, &update_data.buid]).await?;
    conn.query(UPDATE_TITLE, &[&update_data.tuid, &update_data.buid]).await?;
  }

  Ok(HttpResponse::Ok().body("Deleted book."))
}


