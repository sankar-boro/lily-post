use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::Connections;
use scylla::macros::FromRow;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;

#[derive(Deserialize, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    docid: String,
    uniqueId: String,
    category: String,
    metadata: String,
}

pub async fn update(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let docid = Uuid::parse_str(&payload.docid)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let auth = session.user_info()?;
    // let auth_id = Uuid::parse_str(&auth.userId)?;

    let mut batch: Batch = Default::default();
    let blogQuery = Query::from(format!("UPDATE sankar.blog SET title=?, body=?, metadata=? WHERE docid=? AND uniqueId=?"));
    let blogsQuery = Query::from(format!("UPDATE sankar.blogs SET title=?, body=?, metadata=? WHERE docid=?"));
    let userBlogsQuery = Query::from(format!("UPDATE sankar.userblogs SET title=?, body=?, metadata=? WHERE authorId=? AND docid=?"));
    let categoryBlogsQuery = Query::from(format!("UPDATE sankar.categoryblogs SET title=?, body=?, metadata=? WHERE category=? AND docid=?"));

    batch.append_statement(blogQuery);
    batch.append_statement(blogsQuery);
    batch.append_statement(userBlogsQuery);
    batch.append_statement(categoryBlogsQuery);
    app.batch(&batch, (
        (&payload.title, &payload.body, &payload.metadata, &docid, &uniqueId),
        (&payload.title, &payload.body, &payload.metadata, &docid),
        (&payload.title, &payload.body, &payload.metadata, auth.userId, &docid),
        (&payload.title, &payload.body, &payload.metadata, &payload.category, &docid),
    )).await?;

    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
