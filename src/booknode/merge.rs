use crate::{App};
use crate::{auth::AuthSession};
use crate::utils::ParseUuid;
use crate::query::CREATE_BOOK_TITLE;

use scylla::{
    batch::Batch, 
    frame::value::BatchValues,
    BatchResult
};
use uuid::Uuid;
use validator::Validate;
use lily_utils::time_uuid;
use actix_session::Session;
use scylla::macros::FromRow;
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Validate, FromRow)]
pub struct MergeNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    pageId: Option<String>,
    metadata: String,
    topUniqueId: String,
    botUniqueId: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
    pageId: String,
}

pub static UPDATE_PARENT_ID: &str = "UPDATE sankar.book SET parentId=? WHERE bookId=? AND pageId=? AND uniqueId=?";
pub static CHILD: &str = "INSERT INTO sankar.book (
    bookId, pageId, uniqueId, parentId, authorId, title, body, identity, metadata, url, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static UPDATE_PARENT_TITLE: &str = "UPDATE sankar.book_title SET parentId=? WHERE bookId=? AND uniqueId=?";

impl MergeNodeRequest {

    async fn batch(&self, app: &App, batch_values: impl BatchValues) -> Result<BatchResult, crate::AppError> {
        let mut batch: Batch = Default::default();
        batch.append_statement(UPDATE_PARENT_ID);
        batch.append_statement(CHILD);
        batch.append_statement(CREATE_BOOK_TITLE);
        batch.append_statement(UPDATE_PARENT_TITLE);

        Ok(app.batch(&batch, batch_values).await?)
    }

    async fn run(&self, app: &App, session: &Session) -> Result<HttpResponse, crate::AppError> {
        let auth = session.user_info()?;
        // let author_id = Uuid::parse_str(&auth.userId)?;

        // Create and parse elements
        let new_id = time_uuid();
        let new__id = new_id.to_string();
        let book_id = Uuid::parse_str(&self.bookId)?;
        let top_unique_id = Uuid::parse_str(&self.topUniqueId)?;
        let bot_unique_id = Uuid::parse_str(&self.botUniqueId)?;

        let mut image_url = None;
        if let Some(b) = &self.image_url {
            image_url = Some(b.to_owned());
        }

        let mut page_id = None;
        if self.identity == 104 {
            page_id = Some(new_id.clone());
        } else {
            if let Some(pageId) = &self.pageId {
                page_id = Some(pageId.to_uuid()?);
            }
        }
        
        // Create data
        let create_data = ( 
            &book_id,
            &page_id,
            &new_id,
            &top_unique_id,
            auth.userId,
            &self.title,
            &self.body,
            &self.identity,
            &self.metadata,
            &image_url,
            &new_id,
            &new_id
        );

        let update_data = (
            &new_id,
            &book_id,
            &page_id,
            bot_unique_id
        );

        let create_title = (
            &book_id, &top_unique_id, &new_id, &self.title, &self.identity
        );
        let update_title = (
            &new_id,
            &book_id,
            bot_unique_id
        );
        let batch_values = (
            update_data,
            create_data,
            create_title,
            update_title
        );
        
        self.batch(app, batch_values).await?;

        Ok(HttpResponse::Ok().json(Response {
            uniqueId: new__id.clone(),
            pageId: page_id.unwrap().to_string(),
        }))
    }
}

pub async fn merge(
    app: web::Data<App>, 
    payload: web::Json<MergeNodeRequest>,
    session: Session

) 
-> Result<HttpResponse, crate::AppError> 
{   
    payload.run(&app, &session).await
}
