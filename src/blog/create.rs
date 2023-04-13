use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize};
use crate::{
    Connections,
    query::{ CREATE_BLOGS, CREATE_BLOG, CREATE_USER_BLOGS}
};
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;
use serde_json::json;

#[derive(Deserialize, FromRow)]
pub struct ParentRequest {
    title: String,
    body: Option<String>,
    metadata: String,
    image_url: Option<String>,
}

pub async fn create(
    app: web::Data<Connections>, 
    payload: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BLOGS);
    batch.append_statement(CREATE_BLOG);
    batch.append_statement(CREATE_USER_BLOGS);

    let identity: i16 = 101;
    let mut body = String::from("");
    let mut image_url = None;
    if let Some(b) = &payload.body {
        body = b.to_owned();
    }
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }

    let auth = session.user_info()?;
    let timeuid = time_uuid();
    let timeuidstr = timeuid.to_string();

    let batch_values = (
        (&timeuid, auth.userId, &payload.title, &body, &image_url, &payload.metadata, &timeuid, &timeuid),
        (&timeuid, &timeuid, auth.userId, &payload.title, &body, &image_url, &identity, &payload.metadata, &timeuid, &timeuid),
        (&timeuid, auth.userId, &payload.title, &body, &image_url, &payload.metadata, &timeuid, &timeuid)
    );
    app.batch(&batch, &batch_values).await?;

    Ok(
        HttpResponse::Ok().json(json!({
            "blogId": timeuidstr.clone(),
            "uniqueId": timeuidstr.clone(),
            "parentId": null,
            "title": payload.title.clone(),
            "body": body.clone(),
            "url": image_url.clone(),
            "identity": identity,
            "authorId": auth.userId,
            "metadata": payload.metadata.clone(),
            "createdAt": timeuidstr.clone(),
            "updatedAt": timeuidstr.clone(),
        }))
    )
}
