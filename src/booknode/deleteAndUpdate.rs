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
    bookId: String,
    pageId: String,
    updateData: UpdateData,
    bookNodes: Vec<String>,
    titleNodes: Vec<String>,
}

pub async fn deleteAndUpdate(
    app: web::Data<Connections>, 
    payload: web::Json<UpdateOrDelete>
) -> Result<HttpResponse, crate::AppError> {

    let update_data = &payload.updateData;
    let book_id = Uuid::parse_str(&payload.bookId)?;
    let page_id = Uuid::parse_str(&payload.pageId)?;

    let mut batch: Batch = Default::default();

    // update query
    let update_query = Query::new(
        format!("UPDATE sankar.book SET parentId={} WHERE bookId={} AND pageId={} AND uniqueId={}", 
        &update_data.topUniqueId, 
        &book_id,
        &page_id, 
        &update_data.botUniqueId)
    );
    batch.append_statement(update_query); // append query

     // update query
     let update_title_query = Query::new(
        format!("UPDATE sankar.book_title SET parentId={} WHERE bookId={} AND uniqueId={}", 
        &update_data.topUniqueId, 
        &book_id,
        &update_data.botUniqueId)
    );
    batch.append_statement(update_title_query); // append query

    // delete query
    let book_nodes = &payload.bookNodes;
    let mut book_nodes = book_nodes.iter();
    let mut book_node_ids = String::from("");
    if let Some(id) = book_nodes.next() {
        book_node_ids.push_str(id);
    }
    while let Some(id) = book_nodes.next() {
        book_node_ids.push_str(&format!(", {}", &id));
    }
    let delete_nodes_query = Query::new(format!(
        "DELETE FROM sankar.book WHERE bookId={} AND pageId={} AND uniqueId IN ({})",
        &book_id,
        &page_id,
        &book_node_ids)
    );
    batch.append_statement(delete_nodes_query); // append query

    // delete query
    let title_nodes = &payload.titleNodes;
    let mut title_nodes = title_nodes.iter();
    let mut title_node_ids = String::from("");
    if let Some(id) = title_nodes.next() {
        title_node_ids.push_str(id);
    }
    while let Some(id) = title_nodes.next() {
        title_node_ids.push_str(&format!(", {}", &id));
    }
    let delete_title_query = Query::new(format!(
        "DELETE FROM sankar.book_title WHERE bookId={} AND uniqueId IN ({})",
        &book_id,
        &title_node_ids)
    );
    batch.append_statement(delete_title_query); // append query

    app.batch(&batch, ((), (), (), ())).await?;
    Ok(HttpResponse::Ok().body("Updated or deleted."))
}
