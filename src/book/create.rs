use crate::auth::AuthSession;
use crate::{
    Connections, 
    query::{
        CREATE_BOOKS, CREATE_BOOK, CREATE_BOOK_TITLE,
        CREATE_USER_BOOKS, 
        // CREATE_CATEGORY_BOOKS, CREATE_ALLCATEGORY, ADD_USER_CATEGORY
    }
};
use scylla::FromUserType;
use scylla::cql_to_rust::{FromCqlVal};
use scylla::{
    batch::Batch,
    macros::FromRow
};
use serde_json::json;
use validator::Validate;
use actix_session::Session;
use lily_utils::time_uuid;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::create_batch;
// use scylla::prepared_statement::PreparedStatement;

#[derive(Serialize, Deserialize, FromUserType)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    #[validate(length(min = 2))]
    title: String,
    body: Option<String>,
    metadata: String,
    // category: Vec<CategoryData>,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct AddCategory {
    doc_id: String,
    title: String,
    body: String,
    user_id: String,
    createdAt: String,
}

pub async fn create(
    app: web::Data<Connections>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    request.validate()?;
    let auth = session.user_info()?;

    let identity: i16 = 101;
    
    let mut body = String::from("");
    if let Some(b) = &request.body {
        body = b.to_owned();
    }

    let mut image_url = None;
    if let Some(imgurl) = &request.image_url {
        image_url = Some(imgurl.to_owned());
    }
    
    // generate ids
    let timeuid = time_uuid();
    let timeuidstr = timeuid.to_string();
    
    // insert to database
    let batch: Batch = create_batch![CREATE_BOOKS, CREATE_BOOK, CREATE_USER_BOOKS, CREATE_BOOK_TITLE];
    
    let batch_values = (
        // CREATE_BOOKS
        (&timeuid, &auth.userId, &request.title, &body, &image_url, &request.metadata, &timeuid, &timeuid),
        // CREATE_BOOK
        (&timeuid, &timeuid, &timeuid, &auth.userId, &request.title, &body, &image_url, &identity, &request.metadata, &timeuid, &timeuid),
        // CREATE_USER_BOOKS
        (&timeuid, &auth.userId, &request.title, &body, &image_url, &request.metadata, &timeuid, &timeuid),
        // CREATE_BOOK_TITLE
        (&timeuid, &timeuid, &timeuid, &request.title, &identity)
    );
    app.batch(&batch, &batch_values).await?;

    // Prepare the query for later execution
    // let prepared: PreparedStatement = app.session
    // .prepare(CREATE_CATEGORY_BOOKS)
    // .await?;

    // for i in &request.category {
    //     app.execute(&prepared, (&i.category, &timeuid, &auth.userId, &request.title, &body, &image_url, &request.metadata, &timeuid, &timeuid)).await?;
    // }

    // for i in &request.category {
        // app.query(
        //     ADD_USER_CATEGORY, 
        //     (auth.userId, &i.category, &timeuid, &timeuid)
        // ).await?;
        // if !i.exists {
            // app.query(
            //     CREATE_ALLCATEGORY, 
            //     (&i.category, auth.userId, &timeuid, &timeuid)
            // ).await?;
        // }
    // }
    let index = app.indexer.index("books");
    let doc : Vec<AddCategory> = vec![AddCategory {
        doc_id: timeuidstr.to_string(),
        title: request.title.clone(),
        body: body.clone(),
        user_id: auth.userId.to_string(), 
        createdAt: timeuidstr.to_string()
    }];
    index.add_documents(&doc, None).await.unwrap();

    // return response on success
    Ok(
        HttpResponse::Ok().json(json!({
            "bookId": timeuidstr.clone(),
            "pageId": timeuidstr.clone(),
            "uniqueId": timeuidstr.clone(),
            "parentId": null,
            "title": request.title.clone(),
            "body": body.clone(),
            "url": image_url.clone(),
            "identity": identity,
            "authorId": auth.userId,
            "metadata": request.metadata.clone(),
            "createdAt": timeuidstr.clone(),
            "updatedAt": timeuidstr.clone(),
        }))
    )
}


