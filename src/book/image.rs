use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;


#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateBookImage {
    bookId: String,
    uniqueId: String,
    pageId: String,
    category: String,
    image_url: String,
    createdAt: String,
}

pub async fn update_image(
    app: web::Data<App>, 
    payload: web::Json<UpdateBookImage>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let pageId = Uuid::parse_str(&payload.pageId)?;
    let created_at = Uuid::parse_str(&payload.createdAt)?;

    let bookQuery = Query::from(format!("UPDATE sankar.book SET url=? WHERE bookId=? AND pageId=? AND uniqueId=?"));
    let booksQuery = Query::from(format!("UPDATE sankar.books SET url=? WHERE bookId=? AND createdAt=?"));
    let userBooksQuery = Query::from(format!("UPDATE sankar.userbooks SET url=? WHERE authorId=? AND bookId=?"));
    let categoryBooksQuery = Query::from(format!("UPDATE sankar.categorybooks SET url=? WHERE category=? AND bookId=?"));
    
    let mut batch: Batch = Default::default();
    batch.append_statement(bookQuery);
    batch.append_statement(booksQuery);
    batch.append_statement(userBooksQuery);
    batch.append_statement(categoryBooksQuery);
    app.batch(&batch, (
        (&payload.image_url, &bookId, &pageId, &uniqueId), // book
        (&payload.image_url, &bookId, &created_at), // books
        (&payload.image_url, &auth_id, &bookId), // userbooks
        (&payload.image_url, &payload.category, &bookId), // categorybooks
    )).await?;

    Ok(HttpResponse::Ok().json(payload))
}
