use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize};
use crate::{
    App, 
    query::{
        ADD_USER_CATEGORY, DELETE_CATEGORY
    }
};
use validator::Validate;
use scylla::{
    macros::FromRow
};
use crate::auth::AuthSession;

#[derive(Deserialize, Validate, FromRow)]
pub struct UserCategoryRequest {
    category: String,
}

pub async fn add_category(
    app: web::Data<App>,
    request: web::Json<UserCategoryRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let unique_id = time_uuid();
    let _ = app
    .query(ADD_USER_CATEGORY, (auth.userId, &request.category, &unique_id, &unique_id))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}

pub async fn delete_category(
    app: web::Data<App>,
    request: web::Json<UserCategoryRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{

    let auth = session.user_info()?;
    println!("auth: {:?}", auth);
    // let auth_id = Uuid::parse_str(&auth.userId)?;
    let _ = app
    .query(DELETE_CATEGORY, (auth.userId, &request.category))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}
