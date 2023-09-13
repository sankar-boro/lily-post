use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateNode {
    pub blogid: i32,
    pub parentid: i32,
    pub title: String,
    pub body: Option<String>,
    pub identity: i16,
    pub metadata: String,
    pub imageurl: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteBlogRequest {
    pub uid: String,
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct UpdateRequest {
    pub title: String,
    pub body: String,
    pub uid: String,
    pub metadata: String
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct GetBlog {
    pub uid: i32,
    pub authorid: i32,
    pub blogid: i32,
    pub parentid: Option<i32>,
    pub title: String,
    pub body: String,
    pub identity: i16,
    pub metadata: String
}