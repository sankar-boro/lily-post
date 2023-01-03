use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateNodeRequest {
    bookId: String,
    uniqueId: String,
    pageId: String,
    image_url: String,
}

pub async fn update_image(
    app: web::Data<App>, 
    payload: web::Json<UpdateNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;

    let query = format!("UPDATE sankar.book SET url=? WHERE bookId=? AND pageId=? AND uniqueId=?");
    app.query(query, (&payload.image_url, &bookId, &pageId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().json(payload))
}