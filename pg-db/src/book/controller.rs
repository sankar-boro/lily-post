use crate::query::{DELETE_BOOKS, UPDATE_BOOKS};
use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::{ParentRequest, DeleteBookRequest, UpdateRequest};

pub static CREATE_BOOK: &str = "INSERT INTO book (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING uid";

pub static CREATE_BOOK_TITLE: &str = "INSERT INTO title (
    bookid, parentid, title, identity
) VALUES(
    $1, $2, $3, $4
) RETURNING uid";

pub static CREATE_BOOK_NODE: &str = "INSERT INTO booknode (
    uid, authorid, bookid, pageid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
)";

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
    let doc = conn.query(
        CREATE_BOOK, 
        &[
            &auth_id, &payload.title, 
            &payload.body, &image_url, 
            &payload.metadata
        ]
    ).await?;
    let docid: i32 = doc[0].get(0);
    
    let title = conn.query(
        CREATE_BOOK_TITLE, 
        &[
            &docid, &parentid, 
            &payload.title, &identity
        ]
    ).await?;
    let nodeid: i32 = title[0].get(0);

    conn.query(
        CREATE_BOOK_NODE,
        &[
            &nodeid, &auth_id, &docid, &nodeid,
            &parentid, &payload.title, 
            &payload.body, &image_url, 
            &identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": docid,
            "bookid": docid,
            "pageid": nodeid,
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
    let conn = app.get().await?;
    conn.query(DELETE_BOOKS, &[&payload.uid]).await?;

    Ok(HttpResponse::Ok().body("Deleted book."))
}