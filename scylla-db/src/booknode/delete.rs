use uuid::Uuid;
use crate::Connections;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use scylla::query::Query;
use scylla::batch::Batch;

#[derive(Deserialize)]
pub struct DeleteBookNodeRequest {
    docid: String,
    pageId: String,
    bookNodes: Vec<String>,
    titleNodes: Vec<String>,
}

pub async fn delete(
    app: web::Data<Connections>, 
    payload: web::Json<DeleteBookNodeRequest>
) -> Result<HttpResponse, crate::AppError> {
    let book_id = Uuid::parse_str(&payload.docid)?;
    let page_id = Uuid::parse_str(&payload.pageId)?;

    let book_nodes = &payload.bookNodes;
    let mut book_nodes = book_nodes.iter();
    let mut bookNodesIds = String::from("");
    if let Some(id) = book_nodes.next() {
        bookNodesIds.push_str(id);
    }
    while let Some(id) = book_nodes.next() {
        bookNodesIds.push_str(&format!(", {}", &id));
    }

    let titleNodes = &payload.titleNodes;
    let mut titleNodes = titleNodes.iter();
    let mut titleNodeIds = String::from("");
    if let Some(id) = titleNodes.next() {
        titleNodeIds.push_str(id);
    }
    while let Some(id) = titleNodes.next() {
        titleNodeIds.push_str(&format!(", {}", &id));
    }

    let query_book = Query::new(format!("DELETE FROM sankar.book WHERE docid={} AND pageId={} AND uniqueId IN ({})", &book_id, &page_id, &bookNodesIds));
    let query_title = Query::new(format!("DELETE FROM sankar.book_title WHERE docid={} AND uniqueId IN ({})", &book_id, &titleNodeIds));

    let mut batch: Batch = Default::default();
    batch.append_statement(query_book);
    batch.append_statement(query_title);

    let batch_values = ((),());
    // app.query(query, &[]).await?;
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted."))
}
