use crate::query::{CREATE_BOOK, DELETE_BOOKS, UPDATE_BOOKS, CREATE_BOOK_TITLE};
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::{ParentRequest, DeleteBookRequest, UpdateRequest};


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
    
    let mut image_url = None;
    if let Some(imgurl) = &payload.image_url {
        image_url = Some(imgurl.to_owned());
    }

    let conn = app.get().await.unwrap();
    let books = conn.query(CREATE_BOOK, &[&auth_id, &payload.title, &payload.body, &image_url, &payload.metadata]).await.unwrap();
    let book_id: i32 = books[0].get(0);
    conn.query(CREATE_BOOK_TITLE, &[&book_id, &payload.title, &identity]).await.unwrap();
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": book_id,
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
    payload: web::Json<DeleteBookRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    let conn = app.get().await.unwrap();
    conn.query(DELETE_BOOKS, &[&payload.uid]).await.unwrap();

    Ok(HttpResponse::Ok().body("Deleted book."))
}

pub async fn update(
    app: web::Data<Pool>,
    payload: web::Json<UpdateRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    
    let conn = app.get().await.unwrap();
    conn.query(UPDATE_BOOKS, &[&payload.uid, &payload.title, &payload.body, &payload.metadata]).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "uid": &payload.uid,
        "title": &payload.title,
        "body": &payload.body,
        "metadata": &payload.metadata
    })))
}