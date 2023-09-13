use crate::{user, book, blog, booknode, blognode};
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

  config.service(
    web::scope("/blog")
    .route("/create", web::post().to(blog::create))
    .route("/delete", web::post().to(blog::delete))
    .route("/update", web::post().to(blog::update))
  );

  config.service(
    web::scope("/blognode")
    .route("/create", web::post().to(blognode::create))
    .route("/delete", web::post().to(blognode::delete))
    .route("/update", web::post().to(blognode::update))
    .route("/get_all_nodes/{blogid}", web::get().to(blognode::get_all_nodes))
  );
}