use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::Connections;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    docid: String,
    uniqueId: String,
    url: String,
}

pub async fn update_image(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let docid = Uuid::parse_str(&payload.docid)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;

    let query = format!("UPDATE sankar.blog SET url=? WHERE docid=? AND uniqueId=?");
    app.query(query, (&payload.url, &docid, &uniqueId)).await?;
    Ok(HttpResponse::Ok().json(payload))
}