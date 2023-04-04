use actix_web::{HttpResponse, web};
use serde::{Deserialize};
use crate::Connections;
use validator::Validate;
use scylla::macros::FromRow;
use uuid::Uuid;
use scylla::batch::Batch;
use scylla::query::Query;


#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    bookId: String,
    pageId: String,
    uniqueId: String,
    metadata: String,
}

pub async fn update(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;

    let book_query = Query::new(format!("UPDATE sankar.book SET title=?, body=?, metadata=? WHERE bookId=? AND pageId=? AND uniqueId=?"));
    let title_query = Query::new(format!("UPDATE sankar.book_title SET title=? WHERE bookId=? AND uniqueId=?"));

    let mut batch: Batch = Default::default();
    batch.append_statement(book_query);
    batch.append_statement(title_query);

    app.batch(&batch, (
        (&payload.title, &payload.body, &payload.metadata, &bookId, &pageId, &uniqueId), 
        (&payload.title, &bookId, &uniqueId)
    )).await?;

    
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
