use crate::utils::ParseUuid;
use crate::{App, auth::AuthSession};
use crate::query::{CREATE_BOOK_NODE_QUERY};

use uuid::Uuid;
use lily_utils::time_uuid;
use actix_session::Session;
use scylla::macros::FromRow;
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
    metadata: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub async fn create(
    app: web::Data<App>, 
    payload: web::Json<AppendNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    let author_id = Uuid::parse_str(&auth.userId)?;
    let new_id = time_uuid();
    let new_id = new_id.to_string();
    let new__id = new_id.to_string();
    let book_id = &payload.bookId.to_uuid()?;
    let top_unique_id = &payload.topUniqueId.to_uuid()?;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }
    let create_data = ( 
        &book_id,
        &new_id,
        &top_unique_id,
        &author_id,
        &payload.title,
        &payload.body,
        &payload.metadata,
        &image_url,
        &payload.identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BOOK_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: new__id.clone()
    }))
}
