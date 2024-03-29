use crate::{user, book, blog, booknode, blognode};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::signup::signup));
  
  // book
  config.service(
    web::scope("/book")
    .route("/create", web::post().to(book::create))
    .route("/delete", web::post().to(book::delete))
  );

  config.service(
    web::scope("/booknode")
    .route("/create", web::post().to(booknode::create))
    .route("/delete", web::post().to(booknode::delete))
    .route("/update", web::post().to(booknode::update))
    .route("/append", web::post().to(booknode::create))
    .route("/merge", web::post().to(booknode::merge))
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
    .route("/append", web::post().to(blognode::create))
    .route("/merge", web::post().to(blognode::merge))
  );
}