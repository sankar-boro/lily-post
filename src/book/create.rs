use crate::auth::AuthSession;
use crate::{
    Connections, 
    query::{
        CREATE_BOOKS, CREATE_BOOK, 
        CREATE_BOOK_TITLE, CREATE_USER_BOOKS
    }
};
use scylla::FromUserType;
use scylla::cql_to_rust::{FromCqlVal};
use scylla::{
    batch::Batch,
    macros::FromRow
};
use serde_json::json;
use validator::Validate;
use actix_session::Session;
use lily_utils::time_uuid;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::create_batch;

#[derive(Serialize, Deserialize, FromUserType)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    #[validate(length(min = 2))]
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
    payload.validate()?;
    let auth = session.user_info()?;

    let identity: i16 = 101;
    
    let mut body = String::from("");
    if let Some(b) = &payload.body {
        body = b.to_owned();
    }

    let mut image_url = None;
    if let Some(imgurl) = &payload.image_url {
        image_url = Some(imgurl.to_owned());
    }
    
    let timeuid = time_uuid();
    let timeuidstr = timeuid.to_string();
    
    let batch: Batch = create_batch![CREATE_BOOKS, CREATE_BOOK, CREATE_USER_BOOKS, CREATE_BOOK_TITLE];
    
    let batch_values = (
        // CREATE_BOOKS
        (&timeuid, &auth.userId, &payload.title, &body, &image_url, &payload.metadata, &timeuid, &timeuid),
        // CREATE_BOOK
        (&timeuid, &timeuid, &timeuid, &auth.userId, &payload.title, &body, &image_url, &identity, &payload.metadata, &timeuid, &timeuid),
        // CREATE_USER_BOOKS
        (&timeuid, &auth.userId, &payload.title, &body, &image_url, &payload.metadata, &timeuid, &timeuid),
        // CREATE_BOOK_TITLE
        (&timeuid, &timeuid, &timeuid, &payload.title, &identity)
    );
    app.batch(&batch, &batch_values).await?;

    Ok(
        HttpResponse::Ok().json(json!({
            "bookId": timeuidstr.clone(),
            "pageId": timeuidstr.clone(),
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


