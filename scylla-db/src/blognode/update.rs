use uuid::Uuid;
use crate::Connections;
use validator::Validate;
use serde::{Deserialize};
use scylla::macros::FromRow;
use actix_web::{HttpResponse, web};

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    docid: String,
    uniqueId: String,
    metadata: String,
}

pub async fn update(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let docid = Uuid::parse_str(&payload.docid)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let query = format!("UPDATE sankar.blog SET title=?, body=?, metadata=? WHERE docid=? AND uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &payload.metadata, &docid, &uniqueId)).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}