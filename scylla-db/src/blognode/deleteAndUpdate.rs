use uuid::Uuid;
use crate::Connections;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;
use scylla::query::Query;

#[derive(Deserialize)]
struct UpdateData {
    topUniqueId: String,
    botUniqueId: String,
}

#[derive(Deserialize)]
pub struct UpdateOrDelete {
    blogId: String,
    updateData: UpdateData,
    blogNodes: Vec<String>,
}

pub async fn deleteAndUpdate(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateOrDelete>
) -> Result<HttpResponse, crate::AppError> {

    let update_data = &payload.updateData;
    let blog_id = Uuid::parse_str(&payload.blogId)?;

    let mut batch: Batch = Default::default();

    // update query
    let query = Query::new(
        format!("UPDATE sankar.blog SET parentId={} WHERE blogId={} AND uniqueId={}", 
        &update_data.topUniqueId, 
        &blog_id, 
        &update_data.botUniqueId)
    );
    batch.append_statement(query); // append query

    // delete query
    let blogNodes = &payload.blogNodes;
    let mut blogNodes = blogNodes.iter();
    let mut uniqueIds = String::from("");
    if let Some(id) = blogNodes.next() {
        uniqueIds.push_str(id);
    }
    while let Some(id) = blogNodes.next() {
        uniqueIds.push_str(&format!(", {}", &id));
    }
    let query = Query::new(format!(
        "DELETE FROM sankar.blog WHERE blogId={} AND uniqueId IN ({})",
        &blog_id,
        &uniqueIds)
    );
    batch.append_statement(query); // append query

    app.batch(&batch, ((), ())).await?;
    Ok(HttpResponse::Ok().body("Updated or deleted."))
}
