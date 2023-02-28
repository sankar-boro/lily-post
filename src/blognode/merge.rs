use crate::{App};
use crate::{auth::AuthSession};

use scylla::{
    batch::Batch
};
use uuid::Uuid;
use validator::Validate;
use lily_utils::time_uuid;
use actix_session::Session;
use scylla::macros::FromRow;
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Validate, FromRow)]
pub struct MergeNodeRequest {
    title: String,
    body: String,
    blogId: String,
    metadata: String,
    topUniqueId: String,
    botUniqueId: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub static UPDATE_PARENT_ID: &str = "UPDATE sankar.blog SET parentId=? WHERE blogId=? AND uniqueId=?";
pub static CHILD: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, parentId, authorId, title, body, identity, metadata, url, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub async fn merge(
    app: web::Data<App>, 
    payload: web::Json<MergeNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    // let author_id = Uuid::parse_str(&auth.userId)?;

    // Create and parse elements
    let new_id = time_uuid();
    let new__id = new_id.to_string();
    let blog_id = Uuid::parse_str(&payload.blogId)?;
    let top_unique_id = Uuid::parse_str(&payload.topUniqueId)?;
    let bot_unique_id = Uuid::parse_str(&payload.botUniqueId)?;
    let identity: i16 = 104;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }

    // Create data
    let create_data = ( 
        &blog_id,
        &new_id,
        &top_unique_id,
        auth.userId,
        &payload.title,
        &payload.body,
        &identity,
        &payload.metadata,
        &image_url,
        &new_id,
        &new_id
    );

    let update_data = (
        &new_id,
        &blog_id,
        bot_unique_id
    );
    let batch_values = (
        update_data,
        create_data
    );
    let mut batch: Batch = Default::default();
    batch.append_statement(UPDATE_PARENT_ID);
    batch.append_statement(CHILD);
    app.batch(&batch, &batch_values).await?;    

    Ok(HttpResponse::Ok().json(Response {
        uniqueId: new__id.clone()
    }))
}
