use crate::{model::{User, AddUser, UpdateUser}, error::HttpErrorResponse};

use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};

const DB_NAME: &str = "sankar";
const COLL_NAME: &str = "users";

pub async fn add_user(client: web::Data<Client>, form: web::Json<AddUser>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let new_post = doc! { 
        "email": &form.email, 
        "fname": &form.fname, 
        "lname": &form.lname,
        "password": &form.password
    };
    let result = collection.insert_one(new_post, None).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user(client: web::Data<Client>, user_id: web::Path<String>) -> HttpResponse {
    let user_id = match ObjectId::parse_str(user_id.as_str()) {
        Ok(d) => d,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "_id": &user_id }, None).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().body(format!("No post found with postname {user_id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[allow(dead_code)]
pub async fn delete_user(client: web::Data<Client>, user_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let user_id = match ObjectId::parse_str(user_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.delete_one(doc! { "_id": &user_id }, None).await {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_user(client: web::Data<Client>, data: web::Json<UpdateUser>, user_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection: Collection<UpdateUser> = client.database(DB_NAME).collection(COLL_NAME);
    let user_id = match ObjectId::parse_str(&user_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let filter = doc!{ 
        "_id": user_id,
    };
    let update = doc!{ 
        "$set" : { 
            "fname" : &data.fname, 
            "lname": &data.lname,
            "email": &data.email, 
        }
    };

    let result = collection.update_one(filter, update, None).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("post updated")),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}
