use actix_web::{web, HttpResponse};
use mongodb::{bson::doc, bson::oid::ObjectId, Client};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub fname: String,
    pub lname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Post {
    pub _id: ObjectId,
    pub title: String,
    pub body: String,
    pub user_id: ObjectId,
    pub group: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddPost {
    pub title: String,
    pub body: String,
    pub user_id: ObjectId,
    pub group: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddUser {
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdatePost {
    pub _id: String,
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewCollection {
    pub name: String,
    pub dbname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewGroup {
    pub name: String,
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
