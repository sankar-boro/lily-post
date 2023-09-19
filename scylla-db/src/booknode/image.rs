use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::Connections;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    docid: String,
    uniqueId: String,
    pageId: String,
    image_url: String,
}

pub async fn update_image(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let docid = Uuid::parse_str(&payload.docid)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;

    let query = format!("UPDATE sankar.book SET url=? WHERE docid=? AND pageId=? AND uniqueId=?");
    app.query(query, (&payload.image_url, &docid, &pageId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().json(payload))
}