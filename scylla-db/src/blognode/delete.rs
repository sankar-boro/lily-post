use uuid::Uuid;
use crate::Connections;
use serde::Deserialize;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct DeleteNodeRequest {
    docid: String,
    blogNodes: Vec<String>,
}

pub async fn delete(
    app: web::Data<Connections>, 
    payload: web::Json<DeleteNodeRequest>
) -> Result<HttpResponse, crate::AppError> {
    let blog_id = Uuid::parse_str(&payload.docid)?;
    let blogNodes = &payload.blogNodes;
    let mut blogNodes = blogNodes.iter();
    
    let mut uniqueIds = String::from("");
    if let Some(id) = blogNodes.next() {
        uniqueIds.push_str(id);
    }
    while let Some(id) = blogNodes.next() {
        uniqueIds.push_str(&format!(", {}", &id));
    }

    let query = format!("DELETE FROM sankar.blog WHERE docid={} AND uniqueId IN ({})", &blog_id, &uniqueIds);

    app.query(query, &[]).await?;
    Ok(HttpResponse::Ok().body("Deleted."))
}
