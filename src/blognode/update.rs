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
    blogId: String,
    uniqueId: String,
    metadata: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let query = format!("UPDATE sankar.blog SET title=?, body=?, metadata=? WHERE blogId=? AND uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &payload.metadata, &blogId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    blogId: String,
    uniqueId: String,
    url: String,
}

pub async fn update_image_url_node(
    app: web::Data<App>, 
    payload: web::Json<UpdateNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;

    let query = format!("UPDATE sankar.blog SET url=? WHERE blogId=? AND uniqueId=?");
    app.query(query, (&payload.url, &blogId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().json(payload))
}