use crate::{user, book};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::signup::signup));
  
  // book
  config.service(
    web::scope("/book")
    .route("/create", web::post().to(book::create))
  );
}