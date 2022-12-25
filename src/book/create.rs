use crate::auth::AuthSession;
use crate::{
    App, 
    query::{
        CREATE_BOOKS, CREATE_BOOK, CREATE_BOOK_TITLE, CREATE_USER_BOOKS, 
        CREATE_CATEGORY_BOOKS, CREATE_ALLCATEGORY 
    }
};

use scylla::{
    batch::Batch,
    macros::FromRow
};
use uuid::Uuid;
use validator::Validate;
use actix_session::Session;
use lily_utils::time_uuid;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
// use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    title: String,
    body: Option<String>,
    metadata: String,
    category: String,
    image_url: Option<String>,
}

#[derive(Serialize, Validate, FromRow)]
pub struct CreateBookResponse {
    bookId: String,
    pageId: String,
    uniqueId: String,
    parentId: Option<String>,
    authorId: String,
    title: String,
    body: String,
    url: Option<String>,
    identity: i16,
    metadata: String,
    createdAt: String,
    updatedAt: String,
}

pub async fn create(
    app: web::Data<App>,
    // search: web::Data<Mutex<IndexHandler>>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BOOKS);
    batch.append_statement(CREATE_BOOK);
    batch.append_statement(CREATE_USER_BOOKS);
    batch.append_statement(CREATE_CATEGORY_BOOKS);
    batch.append_statement(CREATE_BOOK_TITLE);
    let identity: i16 = 101;

    let mut body = String::from("");
    let mut image_url = None;

    if let Some(b) = &request.body {
        body = b.to_owned();
    }
    if let Some(b) = &request.image_url {
        image_url = Some(b.to_owned());
    }

    if &request.title.len() < &2 {
        return Err(crate::AppError::from("title to small").into());
    }

    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let unique_id = time_uuid();
    let unique__id = unique_id.to_string();
    let batch_values = (
        // CREATE_BOOKS
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // CREATE_BOOK
        (&unique_id, &unique_id, &unique_id, &auth_id, &request.title, &body, &image_url, &identity, &request.metadata, &unique_id, &unique_id),
        // CREATE_USER_BOOKS
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // CREATE_CATEGORY_BOOKS
        (&request.category, &unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // CREATE_BOOK_TITLE
        (&unique_id, &unique_id, &unique_id, &request.title, &identity)
    );

    app.batch(&batch, &batch_values).await?;
    app.query(CREATE_ALLCATEGORY, (&request.category, "demo")).await?;

    // let a = &mut search.try_lock().unwrap();
    // a.create_document(&request.title, &request.body);

    Ok(
        HttpResponse::Ok().json(CreateBookResponse {
            bookId: unique__id.clone(),
            pageId: unique__id.clone(),
            uniqueId: unique__id.clone(),
            parentId: None,
            title: request.title.clone(),
            body: body.clone(),
            url: image_url.clone(),
            identity,
            authorId: auth_id.to_string(),
            metadata: request.metadata.clone(),
            createdAt: unique__id.clone(),
            updatedAt: unique__id.clone(),
        })
    )
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//    userId: String,
//    contextId: String,
//    exp: usize,
// }

// pub async fn create_book_sessionv1(session:Session) -> Result<HttpResponse, crate::AppError> {
//     let auth = session.user_info()?;
//     let userId = auth.userId;
//     let contextId = time_uuid().to_string();

//     let claims = Claims {
//         userId,
//         contextId,
//         exp: 10000000000
//     };

//     let header =
//     Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

//     let token = encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
//     Ok(HttpResponse::Ok().body(token))
// }

