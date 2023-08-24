use crate::{user, book};

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::add_user));
  config.route("/login", web::post().to(user::login));

  config.service(
    web::scope("/user")
    .route("/get/{user_id}", web::get().to(user::get_user))
    .route("/update/{user_id}", web::post().to(user::update_user))
    .route("/delete/{user_id}", web::post().to(user::delete_user))
  );

  config.service(
    web::scope("/book")
    .route("/add_book", web::post().to(book::add_book))
    .route("/get/{book_id}", web::get().to(book::get_book))
    .route("/update/{book_id}", web::post().to(book::update_book))
    .route("/delete/{book_id}", web::post().to(book::delete_book))
  );
}