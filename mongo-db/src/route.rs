use crate::user;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::add_user));

  config.service(
    web::scope("/user")
    .route("/get/{user_id}", web::get().to(user::get_user))
    .route("/update/{user_id}", web::post().to(user::update_user))
    .route("/delete/{user_id}", web::post().to(user::delete_user))
  );
}