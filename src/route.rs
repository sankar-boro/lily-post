use crate::user;
use crate::book;
use crate::blog;
use crate::booknode;
use crate::blognode;
use crate::settings;

use actix_web::{web};
use crate::middleware::Authentication;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::get().to(user::signup));

  // user
  config.service(
    web::scope("/user")
    .wrap(Authentication{})
    .route("/update", web::post().to(user::update))
    .route("/add_category", web::post().to(user::add_category))
    .route("/delete_category", web::post().to(user::delete_category))
  );
  
  config.service(
    web::scope("/book")
    .wrap(Authentication{})
    .route("/create", web::post().to(book::create))
    .route("/delete", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
    .route("/settings/create", web::post().to(settings::create))
    .route("/settings/update", web::post().to(settings::update))
    .route("/update_image", web::post().to(book::update_image))
  );
  
  config.service(
    web::scope("/booknode")
    .wrap(Authentication{})
    .route("/create", web::post().to(booknode::create))
    .route("/merge", web::post().to(booknode::merge))
    .route("/delete", web::post().to(booknode::delete))
    .route("/delete/update", web::post().to(booknode::deleteAndUpdate))
    .route("/update", web::post().to(booknode::update))
    .route("/pull_request", web::post().to(booknode::pull_request))
    .route("/update_image", web::post().to(booknode::update_image))
  );

  config.service(
    web::scope("/blog")
    .wrap(Authentication{})
    .route("/create", web::post().to(blog::create))
    .route("/delete", web::post().to(blog::delete))
    .route("/update", web::post().to(blog::update))
    .route("/update_image", web::post().to(blog::update_image))
  );
  config.service(
    web::scope("/blognode")
    .wrap(Authentication{})
    .route("/create", web::post().to(blognode::create))
    .route("/merge", web::post().to(blognode::merge))
    .route("/delete", web::post().to(blognode::delete))
    .route("/delete/update", web::post().to(blognode::deleteAndUpdate))
    .route("/update", web::post().to(blognode::update))
    .route("/update_image", web::post().to(blognode::update_image))
  );

}