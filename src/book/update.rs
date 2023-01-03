use crate::App;
use uuid::Uuid;
use validator::Validate;
use scylla::batch::Batch;
use scylla::query::Query;
use actix_session::Session;
use scylla::macros::FromRow;
use crate::auth::AuthSession;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Validate, FromRow, Serialize, Clone)]
pub struct UpdateRequest {
    title: String,
    body: String,
    bookId: String,
    uniqueId: String,
    category: String,
    metadata: String,
    createdAt: String,
}

pub async fn update(
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

    let mut batch: Batch = Default::default();
    let bookQuery = Query::from(format!("UPDATE sankar.book SET title=?, body=?, metadata=? WHERE bookId=? AND uniqueId=?"));
    let userBooksQuery = Query::from(format!("UPDATE sankar.userbooks SET title=?, body=?, metadata=? WHERE authorId=? AND bookId=?"));
    let categoryBooksQuery = Query::from(format!("UPDATE sankar.categorybooks SET title=?, body=?, metadata=? WHERE category=? AND bookId=?"));

    batch.append_statement(bookQuery);
    batch.append_statement(userBooksQuery);
    batch.append_statement(categoryBooksQuery);
    app.batch(&batch, (
        (&payload.title, &payload.body, &payload.metadata, &bookId, &uniqueId),
        (&payload.title, &payload.body, &payload.metadata, &auth_id, &bookId),
        (&payload.title, &payload.body, &payload.metadata, &payload.category, &bookId),
    )).await?;
    Ok(HttpResponse::Ok().json(payload.clone()))
}

