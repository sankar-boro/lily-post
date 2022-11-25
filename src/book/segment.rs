use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::App;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;


#[derive(Deserialize)]
pub struct UpdateRequest {
    bookId: String,
    uniqueId: String,
    category: String,
    createdAt: String,
    value: String,
}

pub async fn update_key_value(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let created_at = Uuid::parse_str(&payload.createdAt)?;

    let mut batch: Batch = Default::default();
    let bookQuery = Query::from(format!("UPDATE sankar.book SET url=? WHERE bookId=? AND uniqueId=?"));
    let booksQuery = Query::from(format!("UPDATE sankar.books SET url=? WHERE bookId=? AND createdAt=?"));
    let userBooksQuery = Query::from(format!("UPDATE sankar.userbooks SET url=? WHERE authorId=? AND bookId=?"));
    let categoryBooksQuery = Query::from(format!("UPDATE sankar.categorybooks SET url=? WHERE category=? AND bookId=?"));

    batch.append_statement(bookQuery);
    batch.append_statement(booksQuery);
    batch.append_statement(userBooksQuery);
    batch.append_statement(categoryBooksQuery);
    app.batch(&batch, (
            (&payload.value, &bookId, &uniqueId),
            (&payload.value, &bookId, &created_at),
            (&payload.value, &auth_id, &bookId),
            (&payload.value, &payload.category, &bookId),
        )
    ).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
