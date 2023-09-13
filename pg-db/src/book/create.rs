use crate::{auth::AuthSession, query::CREATE_BOOK};
use deadpool_postgres::Pool;
use serde_json::json;
use validator::Validate;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate)]
pub struct ParentRequest {
    #[validate(length(min = 2))]
    title: String,
    body: Option<String>,
    metadata: String,
    image_url: Option<String>,
}

pub async fn create(
    app: web::Data<Pool>,
    payload: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, Error> 
{
    payload.validate()?;
    // let auth = session.user_info()?;
    let auth_id: i32 = 1;
    let identity: i16 = 101;
    
    let mut image_url = None;
    if let Some(imgurl) = &payload.image_url {
        image_url = Some(imgurl.to_owned());
    }
    
    let conn = app.get().await.unwrap();
    let rows = conn.query(CREATE_BOOK, &[&auth_id, &payload.title, &payload.body, &image_url, &payload.metadata]).await.unwrap();
    let uid: i32 = rows[0].get(0);
    let parentId: Option<i32> = None;

    Ok(
        HttpResponse::Ok().json(json!({
            "uid": uid,
            "parentId": parentId,
            "title": payload.title.clone(),
            "body": payload.body.clone(),
            "url": image_url.clone(),
            "identity": identity,
            "authorId": auth_id,
            "metadata": payload.metadata.clone()
        }))
    )
}


