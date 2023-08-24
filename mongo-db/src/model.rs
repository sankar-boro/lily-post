use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddUser {
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
  pub _id: ObjectId,
  pub email: String,
  pub fname: String,
  pub lname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAllUserData {
  pub _id: ObjectId,
  pub email: String,
  pub fname: String,
  pub lname: String,
  pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdateUser {
  pub email: Option<String>,
  pub fname: Option<String>,
  pub lname: Option<String>,
  pub password: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Book {
    pub _id: ObjectId,
    pub title: String,
    pub body: String,
    pub metadata: String,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddBook {
    pub title: String,
    pub body: String,
    pub metadata: String,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdateBook {
  pub title: Option<String>,
  pub body: Option<String>,
  pub metadata: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewCollection {
    pub name: String,
    pub dbname: String,
}

/// Adds a new user to the "users" collection in the database.
pub async fn create_collection(
    client: web::Data<Client>,
    form: web::Form<NewCollection>,
) -> HttpResponse {
    let collection = client
        .database(&form.dbname)
        .create_collection(&form.name, None)
        .await;
    match collection {
        Ok(_) => HttpResponse::Ok().body(format!("Created collection {}", &form.name)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
