use crate::{
    AUTH_USER,
    model::{
        User, AddUser, UpdateUser, LoginUser, GetAllUserData,
    }, 
    error::HttpErrorResponse
};

use actix_session::Session;
use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use lily_utils::encrypt_text;

const DB_NAME: &str = "sankar";
const COLL_NAME: &str = "users";

pub async fn add_user(client: web::Data<Client>, form: web::Json<AddUser>) -> Result<HttpResponse, HttpErrorResponse> {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let e_text = encrypt_text(&form.password)?;
    let new_user = doc! { 
        "email": &form.email, 
        "fname": &form.fname, 
        "lname": &form.lname,
        "password": e_text
    };
    let res = collection.insert_one(new_user, None).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn login(client: web::Data<Client>, form: web::Json<LoginUser>, session: actix_session::Session) -> Result<HttpResponse, HttpErrorResponse> {
    let e_text = encrypt_text(&form.password)?;
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let get_user: Option<GetAllUserData> = collection.find_one(doc! { "email": &form.email }, None).await?;
    let get_user = match get_user {
        Some(u) => u,
        None => { return Err(HttpErrorResponse::from("User not found".to_string())); }
    };

  if e_text.as_str() == get_user.password.as_str() {
      let login_user = doc! {
            "_id": &get_user._id, 
          "email": &get_user.email, 
          "fname": &get_user.fname, 
          "lname": &get_user.lname
      };
    session.insert(AUTH_USER, &login_user.to_string())?;
    return Ok(HttpResponse::Ok().json(login_user));
  }

  Err(HttpErrorResponse::from("Failed to login user".to_string()))
}

pub async fn get_user(client: web::Data<Client>, user_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let user_id = ObjectId::parse_str(user_id.as_str())?;
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let res = collection.find_one(doc! { "_id": &user_id }, None).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[allow(dead_code)]
pub async fn delete_user(client: web::Data<Client>, user_id: web::Path<String>) -> Result<HttpResponse, HttpErrorResponse> {
    let user_id = match ObjectId::parse_str(user_id.as_str()) {
        Ok(d) => d,
        Err(e) => return Err(HttpErrorResponse::from(e.to_string())),
    };
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.delete_one(doc! { "_id": &user_id }, None).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
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
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Err(HttpErrorResponse::from(err.to_string())),
    }
}

pub async fn user_session(session: Session) 
-> Result<HttpResponse, HttpErrorResponse> {
    let auth_user_session = session.get::<String>("AUTH_USER")?;
    match auth_user_session {
        Some(session) => {
            Ok(HttpResponse::Ok().body(session))
        }
        None => Err(HttpErrorResponse::from("session error".to_string())) 
    }
}