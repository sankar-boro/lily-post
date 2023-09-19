use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::Connections;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;


#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateBookImage {
    docid: String,
    uniqueId: String,
    pageId: String,
    category: String,
    image_url: String,
    createdAt: String,
}

pub async fn update_image(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateBookImage>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    // let auth_id = Uuid::parse_str(&auth.userId)?;
    let docid = Uuid::parse_str(&payload.docid)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;
    let created_at = Uuid::parse_str(&payload.createdAt)?;

    let bookQuery = Query::from(format!("UPDATE sankar.book SET url=? WHERE docid=? AND pageId=? AND uniqueId=?"));
    let booksQuery = Query::from(format!("UPDATE sankar.books SET url=? WHERE docid=? AND createdAt=?"));
    let userBooksQuery = Query::from(format!("UPDATE sankar.userbooks SET url=? WHERE authorId=? AND docid=?"));
    let categoryBooksQuery = Query::from(format!("UPDATE sankar.categorybooks SET url=? WHERE category=? AND docid=?"));
    
    let mut batch: Batch = Default::default();
    batch.append_statement(bookQuery);
    batch.append_statement(booksQuery);
    batch.append_statement(userBooksQuery);
    batch.append_statement(categoryBooksQuery);
    app.batch(&batch, (
        (&payload.image_url, &docid, &pageId, &uniqueId), // book
        (&payload.image_url, &docid, &created_at), // books
        (&payload.image_url, auth.userId, &docid), // userbooks
        (&payload.image_url, &payload.category, &docid), // categorybooks
    )).await?;

    Ok(HttpResponse::Ok().json(payload))
}
