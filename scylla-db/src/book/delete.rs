use crate::Connections;
use crate::auth::AuthSession;

use uuid::Uuid;
use scylla::batch::Batch;
use scylla::query::Query;
use serde::Deserialize;
use actix_session::Session;
use actix_web::{web, HttpResponse};

pub static DELETE_BOOKS: &str = "DELETE FROM sankar.books where bookId=?";
pub static DELETE_USERBOOKS: &str = "DELETE FROM sankar.userbooks where authorId=? AND bookId IN (?)";

#[derive(Deserialize)]
pub struct DeleteBookRequest {
    bookId: String,
    deleteData: Vec<String>,
}

pub async fn delete(
    app: web::Data<Connections>,
    payload: web::Json<DeleteBookRequest>,
    session: Session
) -> Result<HttpResponse, crate::AppError> {
    let book_id = Uuid::parse_str(&payload.bookId)?;
    let auth = session.user_info()?;
    
    let deleteData = &payload.deleteData;
    let mut deleteData = deleteData.iter();
    let mut pageIds = String::from("");
    if let Some(id) = deleteData.next() {
        pageIds.push_str(id);
    }
    while let Some(id) = deleteData.next() {
        pageIds.push_str(&format!(", {}", &id));
    }

    let mut batch: Batch = Default::default();
    batch.append_statement(DELETE_BOOKS);
    let delete_book_query = Query::new(format!("DELETE FROM sankar.book WHERE bookId={} AND pageId IN ({})", &book_id, &pageIds));
    batch.append_statement(delete_book_query);
    batch.append_statement(DELETE_USERBOOKS);
    
    let batch_values = (
        (&book_id,), 
        (), 
        (auth.userId, &book_id,)
    );
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted book."))
}