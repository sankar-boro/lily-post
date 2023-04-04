use crate::{Connections, AppError};

use regex::Regex;
use serde::Serialize;
use serde::{Deserialize};
use validator::{Validate};
use actix_web::{HttpResponse, web};
use lily_utils::{encrypt_text};

lazy_static! {
    static ref MATCH_NAME: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{2,29}$").unwrap();
}
// static INSERT_TABLE__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
// static INSERT_TABLE__USERCREDENTIALS: &str = "INSERT INTO sankar.userCredentials (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT_USER: &str = "INSERT INTO users (fname, lname, email, pwd) VALUES ($1, $2, $3, $4) RETURNING userId";
static INSERT_USER_ADMIN: &str = "INSERT INTO users (userId, fname, lname, email, pwd) VALUES ($1, $2, $3, $4, $5)";


#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(regex = "MATCH_NAME")]
    fname: String,
    #[validate(regex = "MATCH_NAME")]
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Serialize, Debug)]
pub struct AddUserIndex {
    user_id: i32,
    fname: String,
    lname: String
}

pub async fn signup(
    app: web::Data<Connections>, 
    request: web::Json<SignupForm>
) -> Result<HttpResponse, crate::AppError> {
    let client = app.pool.get().await?;

    if let Err(err) = request.validate() {
		return Err(AppError::from(err).into());
	}

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let fname = &request.fname.trim();
    let lname = &request.lname.trim();
    let email = &request.email.trim();
    let stmt = client.prepare_cached(INSERT_USER).await?;
    // let rows = client.query(&stmt, &[fname, lname, email, &password]).await?;
    let rows = client.query_opt(&stmt, &[fname, lname, email, &password]).await?;
    let idx: i32 = rows.unwrap().get(0);

    let index = app.indexer.index("users");
    let doc : Vec<AddUserIndex> = vec![AddUserIndex {
        user_id: idx,
        fname: fname.to_string(),
        lname: lname.to_string(),
    }];
    index.add_documents(&doc, None).await.unwrap();

    Ok(HttpResponse::Ok().body("Ok"))
}

#[derive(Deserialize, Validate, Debug)]
pub struct AdminForm {
    user_id: i32,
    #[validate(regex = "MATCH_NAME")]
    fname: String,
    #[validate(regex = "MATCH_NAME")]
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn signup_admin(
    app: web::Data<Connections>, 
    request: web::Json<AdminForm>
) -> Result<HttpResponse, crate::AppError> {
    let client = app.pool.get().await?;

    if &request.user_id > &1 {
        return Err(AppError::from("cannot create admin").into()); 
    }

    if let Err(err) = request.validate() {
		return Err(AppError::from(err).into());
	}

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let fname = &request.fname.trim();
    let lname = &request.lname.trim();
    let email = &request.email.trim();
    let stmt = client.prepare_cached(INSERT_USER_ADMIN).await?;
    client.query(&stmt, &[&request.user_id, fname, lname, email, &password]).await?;

    let index = app.indexer.index("users");
    let doc : Vec<AddUserIndex> = vec![AddUserIndex {
        user_id: request.user_id.to_owned(),
        fname: fname.to_string(),
        lname: lname.to_string(),
    }];
    index.add_documents(&doc, None).await?;

    Ok(HttpResponse::Ok().body("Ok"))
}
