use crate::user;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::add_user));
  // config.route("/signup_admin", web::post().to(user::signup_admin));

  // config.service(
  //   web::scope("/blognode")
  //   .route("/create", web::post().to(blognode::create))
  // );
}