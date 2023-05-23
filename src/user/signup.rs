use crate::{Connections, AppError};
use crate::client::{self, Method};
use serde_json::{json, Value};

use regex::Regex;
use serde::{Deserialize};
use validator::{Validate};
use actix_web::{HttpResponse, web};
use lily_utils::{encrypt_text};

lazy_static! {
    static ref MATCH_NAME: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{2,29}$").unwrap();
}
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

#[derive(Deserialize, Debug)]
struct ResData {
}

pub async fn signup(
    app: web::Data<Connections>, 
    request: web::Json<SignupForm>
) -> Result<HttpResponse, crate::AppError> {

    request.validate()?;
    let client = app.pool.get().await?;

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let fname = &request.fname.trim();
    let lname = &request.lname.trim();
    let email = &request.email.trim();
    let stmt = client.prepare_cached(INSERT_USER).await?;
    let rows = client.query(&stmt, &[fname, lname, email, &password]).await?;
	let user_id: i32 = rows[0].get(0);

    client::request::<(), Value, ResData>(
        "http://localhost:7705/v2/add_document",
        "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
        Method::Post {
            query: (),
            body: json!({
                "index_name": "users",
                "data": json!({
                    "userId": user_id,
                    "fname": fname,
                    "lname": lname
                }).to_string(),
            }),
        },
        200,
    ).await?;
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
    request.validate()?;

    if &request.user_id > &1 {
        return Err(AppError::from("cannot create admin").into()); 
    }

    let password = match encrypt_text(&request.password) {
        Ok(pass) => {
            if &pass != "$argon2i$v=19$m=4096,t=3,p=1$c2Fua2FyX2Jvcm8$rNL8kMFnoZKT82Qz6ZrwkXBidwXCkPVgA0DH+1f9CKw" {
                return Err(AppError::from("password mismatch").into())
            }
            pass
        },
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let client = app.pool.get().await?;
    
    let fname = &request.fname.trim();
    let lname = &request.lname.trim();
    let email = &request.email.trim();
    let stmt = client.prepare_cached(INSERT_USER_ADMIN).await?;
    let rows = client.query(&stmt, &[&request.user_id, fname, lname, email, &password]).await?;
	let user_id: i32 = rows[0].get(0);

    client::request::<(), Value, String>(
        "http://localhost:7705/v2/add_document",
        "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
        Method::Post {
            query: (),
            body: json!({
                "index_name": "users",
                "data": json!({
                    "userId": user_id,
                    "fname": fname,
                    "lname": lname
                }).to_string(),
            }),
        },
        200,
    ).await?;
    
    Ok(HttpResponse::Ok().body("Ok"))
}
