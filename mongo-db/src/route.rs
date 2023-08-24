use crate::{user, book, book_node};

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::add_user));
  config.route("/login", web::post().to(user::login));

  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
    .route("/get/{user_id}", web::get().to(user::get_user))
    .route("/update/{user_id}", web::post().to(user::update_user))
    .route("/delete/{user_id}", web::post().to(user::delete_user))
  );

  config.route("/books", web::get().to(book::get_all_books));
  config.service(
    web::scope("/book")
    .route("/add_book", web::post().to(book::add_book))
    .route("/get/{book_id}", web::get().to(book::get_book))
    .route("/update/{book_id}", web::post().to(book::update_book))
    .route("/delete/{book_id}", web::post().to(book::delete_book))
  );

  config.service(
    web::scope("/book_node")
    .route("/add_book_node", web::post().to(book_node::add_book_node))
    .route("/get/{book_id}", web::get().to(book_node::get_book_node))
    .route("/update/{book_id}", web::post().to(book_node::update_book_node))
    .route("/delete/{book_id}", web::post().to(book_node::delete_book_node))
  );
}