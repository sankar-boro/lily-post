use crate::{user, book, booknode};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::signup::signup));
  
  // book
  config.service(
    web::scope("/book")
    .route("/create", web::post().to(book::create))
    .route("/delete", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
  );

  config.service(
    web::scope("/booknode")
    .route("/create", web::post().to(booknode::create))
    .route("/delete", web::post().to(booknode::delete))
    .route("/update", web::post().to(booknode::update))
    .route("/get_all_nodes/{bookid}", web::get().to(booknode::get_all_nodes))
  );
}