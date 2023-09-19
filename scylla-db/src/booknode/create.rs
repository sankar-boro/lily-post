use crate::utils::ParseUuid;
use crate::{Connections, auth::AuthSession};
use crate::query::{CREATE_BOOK_NODE_QUERY, CREATE_BOOK_TITLE};

use uuid::Uuid;
use lily_utils::time_uuid;
use actix_session::Session;
use scylla::{
    batch::Batch,
    macros::FromRow
};
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    docid: String,
    pageId: Option<String>,
    topUniqueId: String,
    metadata: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct AddSearchNode {
    docId: String,
    nodeId: String,
    title: String,
    createdAt: String,
    updatedAt: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
    pageId: Option<String>,
}

pub async fn create(
    app: web::Data<Connections>, 
    payload: web::Json<AppendNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;

    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BOOK_NODE_QUERY);
    batch.append_statement(CREATE_BOOK_TITLE);

    let timeuid = time_uuid();
    let mut page_id = None;
    if payload.identity == 104 {
        page_id = Some(timeuid.clone());
    } else {
        if let Some(pageId) = &payload.pageId {
            page_id = Some(pageId.to_uuid()?);
        }
    }

    let timeuidstr = timeuid.to_string();
    let book_id = &payload.docid.to_uuid()?;
    let top_unique_id = &payload.topUniqueId.to_uuid()?;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }
    
    let batch_values = ( 
        (
            &book_id,&page_id,&timeuid,&top_unique_id,&auth.userId,&payload.title,
            &payload.body,&payload.metadata,&image_url,&payload.identity,&timeuid,&timeuid
        ),
        (
            &book_id, &top_unique_id, &timeuid, &payload.title, &payload.identity
        )
    );
    app.batch(&batch, &batch_values).await?;

    Ok(HttpResponse::Ok().json(Response {
        uniqueId: timeuidstr.clone(),
        pageId: page_id.map(|d: Uuid| d.to_string())
    }))
}
