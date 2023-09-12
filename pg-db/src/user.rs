use actix_web::{HttpResponse, web};

pub async fn get_user() -> Result<HttpResponse, crate::error::Error> {
    Ok(HttpResponse::Ok().body("Ok"))
}