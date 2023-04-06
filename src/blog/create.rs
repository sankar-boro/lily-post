use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize, Serialize};
use crate::{
    Connections,
    query::{ CREATE_BLOGS, CREATE_BLOG, CREATE_USER_BLOGS, 
        // CREATE_CATEGORY_BLOGS 
    }
};
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;

#[derive(Deserialize, FromRow)]
pub struct ParentRequest {
    title: String,
    body: Option<String>,
    metadata: String,
    // category: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct ParentResponse {
    blogId: String,
    uniqueId: String,
    parentId: Option<String>,
    authorId: i32,
    title: String,
    body: String,
    url: Option<String>,
    identity: i16,
    metadata: String,
    createdAt: String,
    updatedAt: String,
}

pub async fn create(
    app: web::Data<Connections>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BLOGS);
    batch.append_statement(CREATE_BLOG);
    batch.append_statement(CREATE_USER_BLOGS);
    // batch.append_statement(CREATE_CATEGORY_BLOGS);

    let identity: i16 = 101;
    let mut body = String::from("");
    let mut image_url = None;
    if let Some(b) = &request.body {
        body = b.to_owned();
    }
    if let Some(b) = &request.image_url {
        image_url = Some(b.to_owned());
    }

    let auth = session.user_info()?;
    // let auth_id = &auth.userId.to_uuid()?;
    let unique_id = time_uuid();
    let unique___id = unique_id.to_string();

    let batch_values = (
        (&unique_id, auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        (&unique_id, &unique_id, auth.userId, &request.title, &body, &image_url, &identity, &request.metadata, &unique_id, &unique_id),
        (&unique_id, auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // (&request.category, &unique_id, auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id)
    );
    app.batch(&batch, &batch_values).await?;
    Ok(
        HttpResponse::Ok().json(ParentResponse {
            blogId: unique___id.clone(),
            uniqueId: unique___id.clone(),
            parentId: None,
            title: request.title.clone(),
            body: body.clone(),
            url: image_url.clone(),
            identity,
            authorId: auth.userId,
            metadata: request.metadata.clone(),
            createdAt: unique___id.clone(),
            updatedAt: unique___id.clone(),
        })
    )
}
