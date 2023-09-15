use crate::{model::{GetBook, DeleteBook, AddBook, UpdateBook}, error::HttpErrorResponse};

use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use actix_session::Session;
use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;

const DB_NAME: &str = "sankar";
const COLL_NAME: &str = "books";

#[derive(Serialize, Deserialize)]
struct AuthUser {
  _id: String,
  fname: String,
  lname: String,
}

pub async fn add_book(client: web::Data<Client>, form: web::Json<AddBook>, session: Session) -> Result<HttpResponse, HttpErrorResponse> {
    let auth_user = session.get::<String>("AUTH_USER")?;
    let auth_user = match auth_user {
      Some(a) => a,
      None => { return Ok(HttpResponse::InternalServerError().json(doc!{"status": 500, "data": "Not auth user"}));}
    };
    let auth_user: AuthUser = serde_json::from_str(&auth_user)?;

    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let new_book = doc! { 
        "title": &form.title, 
        "body": &form.body, 
        "metadata": &form.metadata,
        "identity": 101,
        "user_id": &auth_user._id,
    };
    let result = collection.insert_one(new_book, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_all_books(client: web::Data<Client>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection: Collection<GetBook> = client.database(DB_NAME).collection(COLL_NAME);
    let mut cursor = collection.find(None, None).await?;
    let mut books = Vec::new();
    while let Some(book) = cursor.try_next().await? {
        books.push(book);
    }
    Ok(HttpResponse::Ok().json(books))
}

pub async fn get_book(client: web::Data<Client>, book_id: web::Path<String>) -> HttpResponse {
    let book_id = match ObjectId::parse_str(book_id.as_str()) {
        Ok(d) => d,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let collection: Collection<GetBook> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "_id": &book_id }, None).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res),
        Ok(None) => HttpResponse::NotFound().body(format!("No book found with book_id: {book_id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[allow(dead_code)]
pub async fn delete_book(client: web::Data<Client>, book_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let book_id = match ObjectId::parse_str(book_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let collection: Collection<DeleteBook> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.delete_one(doc! { "_id": &book_id }, None).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_book(client: web::Data<Client>, data: web::Json<UpdateBook>, book_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection: Collection<UpdateBook> = client.database(DB_NAME).collection(COLL_NAME);
    let book_id = match ObjectId::parse_str(&book_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let filter = doc!{ 
        "_id": book_id,
    };
    let update = doc!{ 
        "$set" : { 
            "title" : &data.title, 
            "body": &data.body,
            "metadata": &data.metadata,
        }
    };

    let result = collection.update_one(filter, update, None).await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}
