use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use validator::Validate;
use scylla::macros::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    bookId: String,
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

    let query = format!("UPDATE sankar.book SET title=?, body=?, metadata=? WHERE bookId=? AND uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &payload.metadata, &bookId, &uniqueId)).await?;
    
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}


#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    bookId: String,
    uniqueId: String,
    url: String,
}

pub async fn update_image_url_node(
    app: web::Data<App>, 
    payload: web::Json<UpdateNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;

    let query = format!("UPDATE sankar.book SET url=? WHERE bookId=? AND uniqueId=?");
    app.query(query, (&payload.url, &bookId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().json(payload))
}