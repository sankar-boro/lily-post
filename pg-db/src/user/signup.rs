use deadpool_postgres::Pool;

use regex::Regex;
use serde::Deserialize;
use validator::Validate;
use crate::{error::Error, query::SIGNUP};
use lily_utils::encrypt_text;
use serde_json::json;
use actix_web::{HttpResponse, web};

lazy_static! {
    static ref MATCH_NAME: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{2,29}$").unwrap();
}

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

pub async fn signup(
    app: web::Data<Pool>, 
    request: web::Json<SignupForm>
) -> Result<HttpResponse, Error> {

    request.validate()?;
    
    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(Error::from(err).into())
    };
    
    let fname = request.fname.trim();
    let lname = request.lname.trim();
    let email = request.email.trim();
    
    let conn = app.get().await.unwrap();
    let rows = conn.query(SIGNUP, &[&fname, &lname, &email, &password]).await.unwrap();
    let uid: i32 = rows[0].get(0);

    Ok(HttpResponse::Ok().json(json!({ "uid": uid, "fname": &fname, "lname": &lname, "email": &email })))
}
