use crate::user;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::get().to(user::signup::signup));
  // config.route("/signup_admin", web::post().to(user::signup_admin));

  // user
  // config.service(
  //   web::scope("/user")
  //   .route("/update", web::post().to(user::update))
  // );
}