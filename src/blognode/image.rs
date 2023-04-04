use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::Connections;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    blogId: String,
    uniqueId: String,
    url: String,
}

pub async fn update_image(
    app: web::Data<Connections>, 
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