use crate::Connections;
use crate::utils::ParseUuid;
use crate::{auth::AuthSession};
use crate::query::{CREATE_BLOG_NODE_QUERY};

use lily_utils::time_uuid;
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use validator::Validate;
use scylla::macros::FromRow;
use actix_session::Session;

#[derive(Deserialize, Validate, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    blogId: String,
    topUniqueId: String,
    metadata: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub async fn create(
    app: web::Data<Connections>, 
    payload: web::Json<AppendNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    // let author_id = Uuid::parse_str(&auth.userId)?;

    let identity: i16 = 104;
    let new_id = time_uuid();
    let new__id = new_id.to_string();
    let blog_id = payload.blogId.to_uuid()?;
    let top_unique_id = payload.topUniqueId.to_uuid()?;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }
    let create_data = ( 
        &blog_id,
        &new_id,
        &top_unique_id,
        auth.userId,
        &payload.title,
        &payload.body,
        &payload.metadata,
        &image_url,
        identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BLOG_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: new__id.clone()
    }))
}
