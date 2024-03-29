use actix_web::{web, HttpResponse};
use crate::Connections;

#[allow(dead_code)]
pub async fn delete_one(session: web::Data<Connections>, path: web::Path<i32>) -> Result<HttpResponse, crate::AppError> {
    // let user_id =  Uuid::parse_str(&id)?;
    let user_id = path.into_inner();
    
    session
    .query("DELETE FROM sankar.users WHERE id=?", (user_id,))
    .await?;
    Ok(HttpResponse::Ok().body("User deleted"))
}
