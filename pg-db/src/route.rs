use actix_web::{web};
use crate::user;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/get_user", web::get().to(user::get_user));
  // config.route("/signup_admin", web::post().to(user::signup_admin));

  // user
  // config.service(
  //   web::scope("/user")
  //   .route("/update", web::post().to(user::update))
  // );
}