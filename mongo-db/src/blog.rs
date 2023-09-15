use crate::{model::{GetBlog, DeleteBlog, AddBlog, UpdateBlog}, error::HttpErrorResponse};

use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use actix_session::Session;
use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;

const DB_NAME: &str = "sankar";
const COLL_NAME: &str = "blogs";

#[derive(Serialize, Deserialize)]
struct AuthUser {
  _id: String,
  fname: String,
  lname: String,
}

pub async fn add_blog(client: web::Data<Client>, form: web::Json<AddBlog>, session: Session) -> Result<HttpResponse, HttpErrorResponse> {
    let auth_user = session.get::<String>("AUTH_USER")?;
    let auth_user = match auth_user {
      Some(a) => a,
      None => { return Ok(HttpResponse::InternalServerError().json(doc!{"status": 500, "data": "Not auth user"}));}
    };
    let auth_user: AuthUser = serde_json::from_str(&auth_user)?;

    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let new_blog = doc! { 
        "title": &form.title, 
        "body": &form.body, 
        "metadata": &form.metadata,
        "identity": 101,
        "user_id": &auth_user._id,
    };
    let result = collection.insert_one(new_blog, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_all_blogs(client: web::Data<Client>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection: Collection<GetBlog> = client.database(DB_NAME).collection(COLL_NAME);
    let mut cursor = collection.find(None, None).await?;
    let mut blogs = Vec::new();
    while let Some(blog) = cursor.try_next().await? {
        blogs.push(blog);
    }
    Ok(HttpResponse::Ok().json(blogs))
}

pub async fn get_blog(client: web::Data<Client>, blog_id: web::Path<String>) -> HttpResponse {
    let blog_id = match ObjectId::parse_str(blog_id.as_str()) {
        Ok(d) => d,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let collection: Collection<GetBlog> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "_id": &blog_id }, None).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res),
        Ok(None) => HttpResponse::NotFound().body(format!("No blog found with blog_id: {blog_id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[allow(dead_code)]
pub async fn delete_blog(client: web::Data<Client>, blog_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let blog_id = match ObjectId::parse_str(blog_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let collection: Collection<DeleteBlog> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.delete_one(doc! { "_id": &blog_id }, None).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_blog(client: web::Data<Client>, data: web::Json<UpdateBlog>, blog_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection: Collection<UpdateBlog> = client.database(DB_NAME).collection(COLL_NAME);
    let blog_id = match ObjectId::parse_str(&blog_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let filter = doc!{ 
        "_id": blog_id,
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
