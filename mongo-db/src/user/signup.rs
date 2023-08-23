use crate::model::AddUser;

use actix_web::{web, HttpResponse};
use mongodb::{bson::doc, Client};

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
        Ok(_) => HttpResponse::Ok().body("post added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
