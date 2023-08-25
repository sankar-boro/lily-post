use crate::{model::{GetBlogNode, DeleteBlogNode, AddBlogNode, UpdateBlog}, error::HttpErrorResponse};

use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};

const DB_NAME: &str = "sankar";
const COLL_NAME: &str = "blog_node";

pub async fn add_blog_node(client: web::Data<Client>, form: web::Json<AddBlogNode>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let new_blog = doc! { 
        "title": &form.title, 
        "body": &form.body, 
        "metadata": &form.metadata,
        "identity": &form.identity,
        "blog_id": &form.blog_id,
        "parent_id": &form.parent_id,
    };
    let result = collection.insert_one(new_blog, None).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_blog_node(client: web::Data<Client>, blog_id: web::Path<String>) -> HttpResponse {
    let blog_id = match ObjectId::parse_str(blog_id.as_str()) {
        Ok(d) => d,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let collection: Collection<GetBlogNode> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "_id": &blog_id }, None).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res),
        Ok(None) => HttpResponse::NotFound().body(format!("No blog found with blog_id: {blog_id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[allow(dead_code)]
pub async fn delete_blog_node(client: web::Data<Client>, blog_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let blog_id = match ObjectId::parse_str(blog_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let collection: Collection<DeleteBlogNode> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.delete_one(doc! { "_id": &blog_id }, None).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_blog_node(client: web::Data<Client>, data: web::Json<UpdateBlog>, blog_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
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
