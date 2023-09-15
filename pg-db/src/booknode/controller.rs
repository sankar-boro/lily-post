use crate::query::{CREATE_BOOK_NODE, DELETE_BOOKS, UPDATE_BOOKS, CREATE_BOOK_TITLE, BOOK_DATA};
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::{CreateNode, GetBook, DeleteBookRequest, UpdateRequest};

pub async fn get_all_nodes(
    app: web::Data<Pool>,
    path: web::Path<String>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
    let bookid: i32 = path.parse()?;
    let conn = app.get().await?;
    let books = conn.query(
        BOOK_DATA, 
        &[&bookid]
    ).await?;

    let mut allbooks = Vec::new();
    for i in 0..books.len() {
        allbooks.push(GetBook {
            uid: books[i].get(0),
            authorid: books[i].get(1),
            bookid: books[i].get(2),
            parentid: books[i].get(3),
            title: books[i].get(4),
            body: books[i].get(5),
            identity: books[i].get(6),
            metadata: books[i].get(7)
        });

    }

    Ok(HttpResponse::Ok().json(allbooks))
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
        CREATE_BOOK_TITLE, 
        &[
            &payload.bookid, &payload.parentid, 
            &payload.title, &payload.identity
        ]
    ).await?;
    conn.query(
        CREATE_BOOK_NODE, 
        &[
            &auth_id, &payload.bookid, 
            &payload.parentid, &payload.title, 
            &payload.body, &payload.imageurl, 
            &payload.identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": payload.bookid,
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
    payload: web::Json<DeleteBookRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    let conn = app.get().await?;
    conn.query(DELETE_BOOKS, &[&payload.uid]).await?;

    Ok(HttpResponse::Ok().body("Deleted book."))
}

pub async fn update(
    app: web::Data<Pool>,
    payload: web::Json<UpdateRequest>,
    _: Session
) -> Result<HttpResponse, Error> {
    
    let conn = app.get().await?;
    conn.query(UPDATE_BOOKS, &[&payload.uid, &payload.title, &payload.body, &payload.metadata]).await?;

    Ok(HttpResponse::Ok().json(json!({
        "uid": &payload.uid,
        "title": &payload.title,
        "body": &payload.body,
        "metadata": &payload.metadata
    })))
}