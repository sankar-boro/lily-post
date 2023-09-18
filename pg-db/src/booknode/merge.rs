use deadpool_postgres::Pool;
use serde_json::json;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::error::Error;
use super::model::CreateNode;


pub static CREATE_BOOK_TITLE: &str = "INSERT INTO title (
    bookid, parentid, title, identity
) VALUES(
    $1, $2, $3, $4
) RETURNING uid, identity";
pub static CREATE_BOOK_NODE: &str = "INSERT INTO booknode (
    authorid, bookid, pageid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8, $9
)";
pub async fn merge(
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
    let row = conn.query(
        CREATE_BOOK_TITLE, 
        &[
            &payload.docid, &payload.tuid, 
            &payload.title, &payload.identity
        ]
    ).await?;

    let row_id: i32 = match payload.pageid {
        Some(row_id) => row_id,
        None => { let xid: i32 = row[0].get(0); xid }
    };
    
    conn.query(
        CREATE_BOOK_NODE, 
        &[
            &auth_id, &payload.docid, &row_id, 
            &payload.tuid, &payload.title, 
            &payload.body, &payload.imageurl, 
            &payload.identity, &payload.metadata
        ]
    ).await?;
    
    Ok(
        HttpResponse::Ok().json(json!({
            "uid": payload.docid,
            "parentId": payload.tuid,
            "title": payload.title.clone(),
            "body": payload.body.clone(),
            "url": image_url.clone(),
            "identity": &payload.identity,
            "authorId": auth_id,
            "metadata": payload.metadata.clone()
        }))
    )
}
