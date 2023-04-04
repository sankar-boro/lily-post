use crate::auth::AuthSession;
use crate::{
    Connections, 
    query::{
        CREATE_BOOKS, CREATE_BOOK, CREATE_BOOK_TITLE, ADD_USER_CATEGORY,
        CREATE_USER_BOOKS, CREATE_CATEGORY_BOOKS, CREATE_ALLCATEGORY
    }
};
use scylla::FromUserType;
use scylla::cql_to_rust::{FromCqlVal};
use scylla::prepared_statement::PreparedStatement;
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
    category: Vec<CategoryData>,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct AddCategory {
    id: uuid::Uuid,
    name: String,
}

pub async fn create(
    app: web::Data<Connections>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    // validate
    if let Err(err) = request.validate() {
		return Err(crate::AppError::from(err).into());
	}

    // init variables
    let identity: i16 = 101;
    let mut body = String::from("");
    let mut image_url = None;
    
    // what if data is null
    if let Some(b) = &request.body {
        body = b.to_owned();
    }
    if let Some(b) = &request.image_url {
        image_url = Some(b.to_owned());
    }
    
    // generate ids
    let auth = session.user_info()?;
    let unique_id = time_uuid();
    let unique__id = unique_id.to_string();
    
    // insert to database
    let batch: Batch = create_batch![CREATE_BOOKS, CREATE_BOOK, CREATE_USER_BOOKS, CREATE_BOOK_TITLE];
    
    let batch_values = (
        // CREATE_BOOKS
        (&unique_id, &auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // CREATE_BOOK
        (&unique_id, &unique_id, &unique_id, &auth.userId, &request.title, &body, &image_url, &identity, &request.metadata, &unique_id, &unique_id),
        // CREATE_USER_BOOKS
        (&unique_id, &auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        // CREATE_BOOK_TITLE
        (&unique_id, &unique_id, &unique_id, &request.title, &identity)
    );
    app.batch(&batch, &batch_values).await?;

    // Prepare the query for later execution
    let prepared: PreparedStatement = app.session
    .prepare(CREATE_CATEGORY_BOOKS)
    .await?;

    for i in &request.category {
        app.execute(&prepared, (&i.category, &unique_id, &auth.userId, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id)).await?;
    }

    let uunique_id = time_uuid();

    for i in &request.category {
        app.query(
            ADD_USER_CATEGORY, 
            (auth.userId, &i.category, &uunique_id, &uunique_id)
        ).await?;
        if !i.exists {
            app.query(
                CREATE_ALLCATEGORY, 
                (&i.category, auth.userId, &uunique_id, &uunique_id)
            ).await?;
            let index = app.indexer.index("categories");
            let doc : Vec<AddCategory> = vec![AddCategory {
                id: uunique_id.clone(),
                name: i.category.to_owned(),
            }];
            index.add_documents(&doc, None).await.unwrap();
        }
    }

    // return response on success
    Ok(
        HttpResponse::Ok().json(json!({
            "bookId": unique__id.clone(),
            "pageId": unique__id.clone(),
            "uniqueId": unique__id.clone(),
            "parentId": null,
            "title": request.title.clone(),
            "body": body.clone(),
            "url": image_url.clone(),
            "identity": identity,
            "authorId": auth.userId,
            "metadata": request.metadata.clone(),
            "createdAt": unique__id.clone(),
            "updatedAt": unique__id.clone(),
        }))
    )
}


