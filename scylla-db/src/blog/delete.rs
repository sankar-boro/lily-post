use uuid::Uuid;
use crate::Connections;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;
use actix_session::Session;
use crate::auth::AuthSession;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct DeleteBlogRequest {
    blogId: String
}

pub static DELETE_BLOGS: &str = "DELETE FROM sankar.blogs where blogId=?";
pub static DELETE_BLOG: &str = "DELETE FROM sankar.blog where blogId=?";
pub static DELETE_USERBLOGS: &str = "DELETE FROM sankar.userblogs where authorId=? AND blogId IN (?)";

pub async fn delete(
    app: web::Data<Connections>,
    payload: web::Json<DeleteBlogRequest>,
    session: Session
) -> Result<HttpResponse, crate::AppError> {
    let blog_id = Uuid::parse_str(&payload.blogId)?;
    let auth = session.user_info()?;

    let mut batch: Batch = Default::default();
    batch.append_statement(DELETE_BLOGS);
    batch.append_statement(DELETE_BLOG);
    batch.append_statement(DELETE_USERBLOGS);
    
    let batch_values = (
        (&blog_id,), 
        (&blog_id,), 
        (auth.userId, &blog_id,)
    );
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted blog."))
}