use crate::query::{CREATE_BLOG_NODE, DELETE_BLOGS, UPDATE_BLOGS, BLOG_DATA};
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::{CreateNode, GetBlog, DeleteBlogRequest, UpdateRequest};

pub async fn get_all_nodes(
    app: web::Data<Pool>,
    path: web::Path<String>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
    let blogid: i32 = path.parse()?;
    let conn = app.get().await?;
    let blogs = conn.query(
        BLOG_DATA, 
        &[&blogid]
    ).await?;

    let mut allblogs = Vec::new();
    for i in 0..blogs.len() {
        allblogs.push(GetBlog {
            uid: blogs[i].get(0),
            authorid: blogs[i].get(1),
            blogid: blogs[i].get(2),
            parentid: blogs[i].get(3),
            title: blogs[i].get(4),
            body: blogs[i].get(5),
            identity: blogs[i].get(6),
            metadata: blogs[i].get(7)
        });

    }

    Ok(HttpResponse::Ok().json(allblogs))
} 

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
            &auth_id, &payload.blogid, 
            &payload.parentid, &payload.title, 
            &payload.body, &payload.imageurl, 
            &payload.identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": payload.blogid,
            "parentId": payload.parentid,
            "title": payload.title.clone(),
            "body": payload.body.clone(),
            "url": image_url.clone(),
            "identity": &payload.identity,
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
    conn.query(
        UPDATE_BLOGS, 
        &[
            &payload.uid, &payload.title, 
            &payload.body, &payload.metadata
        ]
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "uid": &payload.uid,
        "title": &payload.title,
        "body": &payload.body,
        "metadata": &payload.metadata
    })))
}