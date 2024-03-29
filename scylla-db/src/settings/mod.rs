use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize};
use crate::Connections;
use crate::query::{CREATE_USER_BOOK_SETTINGS, UPDATE_USER_BOOK_SETTINGS};
use validator::Validate;
use scylla::{
    macros::FromRow
};
use crate::auth::AuthSession;
use crate::utils::ParseUuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    docid: String,
    settings: String,
}

pub async fn create(
    app: web::Data<Connections>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    // let author_id = Uuid::parse_str(&auth.userId)?;
    let book_id = &request.docid.to_uuid()?;

    let create_data = ( 
        auth.userId,
        &book_id,
        &request.settings,
    );
    app.query(CREATE_USER_BOOK_SETTINGS, create_data).await?;
    Ok(HttpResponse::Ok().body("Ok."))
}

pub async fn update(
    app: web::Data<Connections>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    // let author_id = Uuid::parse_str(&auth.userId)?;
    let book_id = &request.docid.to_uuid()?;

    let create_data = ( 
        &request.settings,
        auth.userId,
        &book_id,
    );
    app.query(UPDATE_USER_BOOK_SETTINGS, create_data).await?;
    Ok(HttpResponse::Ok().body("Ok."))
}
