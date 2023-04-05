use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize};
use crate::{
    Connections, 
    query::{
        FOLLOW_USER, UNFOLLOW_USER
    }
};
use validator::Validate;
use scylla::{
    macros::FromRow
};
use crate::auth::AuthSession;

#[derive(Deserialize, Validate, FromRow)]
pub struct User {
    user_id: String
}

pub async fn follow(
    app: web::Data<Connections>,
    request: web::Json<User>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let unique_id = time_uuid();
    let _ = app
    .query(FOLLOW_USER, (auth.userId, &request.user_id, &unique_id, &unique_id))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}

pub async fn unfollow(
    app: web::Data<Connections>,
    request: web::Json<User>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{

    let auth = session.user_info()?;
    let _ = app
    .query(UNFOLLOW_USER, (auth.userId, &request.user_id))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}
