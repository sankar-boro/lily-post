use actix_web::{HttpResponse, web};
use serde::{Deserialize};
use crate::App;
use validator::Validate;
use scylla::macros::FromRow;
use uuid::Uuid;

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
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;

    let query = format!("UPDATE sankar.book SET title=?, body=?, metadata=? WHERE bookId=? AND pageId=? uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &payload.metadata, &bookId, &pageId, &uniqueId)).await?;
    
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
