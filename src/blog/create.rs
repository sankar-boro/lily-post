use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize, Serialize};
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
    // category: String,
    image_url: Option<String>,
}

// #[derive(Serialize)]
// pub struct ParentResponse {
//     blogId: String,
//     uniqueId: String,
//     parentId: Option<String>,
//     authorId: i32,
//     title: String,
//     body: String,
//     url: Option<String>,
//     identity: i16,
//     metadata: String,
//     createdAt: String,
//     updatedAt: String,
// }

#[derive(Serialize)]
struct AddDoc {
    docId: String,
    userId: String,
    title: String,
    body: String,
    uname: String,
    metadata: String,
    createdAt: String,
    updatedAt: String,
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
    // batch.append_statement(CREATE_CATEGORY_BLOGS);

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
        (&timeuid, auth.userId, &payload.title, &body, &image_url, &payload.metadata, &timeuid, &timeuid),
        // (&payload.category, &unique_id, auth.userId, &payload.title, &body, &image_url, &payload.metadata, &unique_id, &unique_id)
    );
    app.batch(&batch, &batch_values).await?;

    let index = app.indexer.index("blogs");
    let search_book : Vec<AddDoc> = vec![AddDoc {
        docId: timeuidstr.to_string(),
        userId: auth.userId.to_string(),
        title: payload.title.clone(),
        body: body.clone(),
        uname: format!("{} {}", &auth.fname, &auth.lname),
        metadata: payload.metadata.clone(),
        createdAt: timeuid.to_string(),
        updatedAt: timeuid.to_string()
    }];
    index.add_documents(&search_book, None).await.unwrap();

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
