use crate::query::{CREATE_BLOG, DELETE_BLOGS, UPDATE_BLOGS, CREATE_BLOG_NODE};
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::{ParentRequest, DeleteBlogRequest, UpdateRequest};

pub async fn create(
    app: web::Data<Pool>,
    payload: web::Json<ParentRequest>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
    payload.validate()?;
    // let auth = session.user_info()?;
    let auth_id: i32 = 1;
    let identity: i16 = 101;
    let parentid: Option<i32> = None;

    let mut image_url = None;
    if let Some(imgurl) = &payload.image_url {
        image_url = Some(imgurl.to_owned());
    }

    let conn = app.get().await?;
    let blog = conn.query(
        CREATE_BLOG, 
        &[
            &auth_id, &payload.title, 
            &payload.body, &image_url, 
            &payload.metadata
        ]
    ).await?;
    let docid: i32 = blog[0].get(0);
    conn.query(
        CREATE_BLOG_NODE,
        &[
            &auth_id, &docid, 
            &parentid, &payload.title, 
            &payload.body, &image_url, 
            &identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": docid,
            "parentId": null,
            "title": payload.title.clone(),
            "body": payload.body.clone(),
            "url": image_url.clone(),
            "identity": identity,
            "authorId": auth_id,
            "metadata": payload.metadata.clone()
        }))
    )
}

pub async fn delete(
    app: web::Data<Pool>,
    payload: web::Json<DeleteBlogRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    let conn = app.get().await?;
    conn.query(DELETE_BLOGS, &[&payload.uid]).await?;

    Ok(HttpResponse::Ok().body("Deleted blog."))
}

pub async fn update(
    app: web::Data<Pool>,
    payload: web::Json<UpdateRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    
    let conn = app.get().await?;
    conn.query(UPDATE_BLOGS, &[&payload.uid, &payload.title, &payload.body, &payload.metadata]).await?;

    Ok(HttpResponse::Ok().json(json!({
        "uid": &payload.uid,
        "title": &payload.title,
        "body": &payload.body,
        "metadata": &payload.metadata
    })))
}