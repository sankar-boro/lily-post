use crate::user;
use crate::book;
use crate::blog;
use crate::booknode;
use crate::blognode;
use crate::settings;

use actix_web::{web};

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/signup", web::post().to(user::signup));
  config.route("/signup_admin", web::post().to(user::signup_admin));

  // user
  config.service(
    web::scope("/user")
    .route("/update", web::post().to(user::update))
    .route("/add_category", web::post().to(user::add_category))
    .route("/delete_category", web::post().to(user::delete_category))
    .route("/follow", web::post().to(user::follow))
    .route("/unfollow", web::post().to(user::unfollow))
  );
  
  config.service(
    web::scope("/book")
    .route("/create", web::post().to(book::create))
    .route("/delete", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
    .route("/settings/create", web::post().to(settings::create))
    .route("/settings/update", web::post().to(settings::update))
    .route("/update_image", web::post().to(book::update_image))
  );
  
  config.service(
    web::scope("/booknode")
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
    .route("/create", web::post().to(blog::create))
    .route("/delete", web::post().to(blog::delete))
    .route("/update", web::post().to(blog::update))
    .route("/update_image", web::post().to(blog::update_image))
  );
  config.service(
    web::scope("/blognode")
    .route("/create", web::post().to(blognode::create))
    .route("/merge", web::post().to(blognode::merge))
    .route("/delete", web::post().to(blognode::delete))
    .route("/delete/update", web::post().to(blognode::deleteAndUpdate))
    .route("/update", web::post().to(blognode::update))
    .route("/update_image", web::post().to(blognode::update_image))
  );
}