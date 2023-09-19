use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::CreateNode;


pub static CREATE_BLOG_NODE: &str = "INSERT INTO blognode (
    authorid, docid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

pub async fn create(
    app: web::Data<Pool>,
    payload: web::Json<CreateNode>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
    payload.validate()?;
    // let auth = session.user_info()?;
    let auth_id: i32 = 1;
    
    let mut image_url = None;
    if let Some(imgurl) = &payload.imageurl {
        image_url = Some(imgurl.to_owned());
    }

    let conn = app.get().await?;
    conn.query(
        CREATE_BLOG_NODE, 
        &[
            &auth_id, &payload.docid, 
            &payload.tuid, &payload.title, 
            &payload.body, &payload.imageurl, 
            &payload.identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": payload.docid,
            "parentid": payload.tuid,
            "title": payload.title.clone(),
            "body": payload.body.clone(),
            "url": image_url.clone(),
            "identity": &payload.identity,
            "authorid": auth_id,
            "metadata": payload.metadata.clone()
        }))
    )
}