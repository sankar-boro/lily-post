use crate::App;
use crate::AppError;

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

#[derive(Serialize)]
pub struct AddUserIndex {
    id: i32,
    fname: String,
    lname: String
}

pub async fn signup(
    app: web::Data<App>, 
    request: web::Json<SignupForm>
) -> Result<HttpResponse, crate::AppError> {
    let client = app.pool.get().await?;
    let healthy = app.indexer.is_healthy().await;
    if !healthy {
        return Err(AppError::from("indexer not healthy").into()); 
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
    let stmt = client.prepare_cached(INSERT_USER).await?;
    // let rows = client.query(&stmt, &[fname, lname, email, &password]).await?;
    let rows = client.query_opt(&stmt, &[fname, lname, email, &password]).await?;
    let idx: i32 = rows.unwrap().get(0);

    let index = app.indexer.index("users");
    let doc : Vec<AddUserIndex> = vec![AddUserIndex {
        id: idx,
        fname: fname.to_string(),
        lname: lname.to_string(),
    }];
    index.add_documents(&doc, None).await.unwrap();

    Ok(HttpResponse::Ok().body("Ok"))
}
