use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;


#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateRequest {
    blogId: String,
    uniqueId: String,
    category: String,
    createdAt: String,
    url: String,
}

pub async fn update_image(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let created_at = Uuid::parse_str(&payload.createdAt)?;

    let mut batch: Batch = Default::default();
    let blogQuery = Query::from(format!("UPDATE sankar.blog SET url=? WHERE blogId=? AND uniqueId=?"));
    let userBlogsQuery = Query::from(format!("UPDATE sankar.userblogs SET url=? WHERE authorId=? AND blogId=?"));
    let categoryBlogsQuery = Query::from(format!("UPDATE sankar.categoryblogs SET url=? WHERE category=? AND blogId=?"));

    batch.append_statement(blogQuery);
    batch.append_statement(userBlogsQuery);
    batch.append_statement(categoryBlogsQuery);
    app.batch(&batch, (
            (&payload.url, &blogId, &uniqueId),
            (&payload.url, &blogId, &created_at),
            (&payload.url, &auth_id, &blogId),
            (&payload.url, &payload.category, &blogId),
        )
    ).await?;
    Ok(HttpResponse::Ok().json(payload))
}
