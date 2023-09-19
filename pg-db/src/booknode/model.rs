use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateNode {
    pub docid: i32,
    pub pageid: Option<i32>,
    pub tuid: Option<i32>,
    pub buid: Option<i32>,
    pub title: String,
    pub body: Option<String>,
    pub identity: i16,
    pub metadata: String,
    pub imageurl: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteBookRequest {
    pub uid: String,
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct UpdateRequest {
    pub title: String,
    pub body: String,
    pub uid: i32,
    pub docid: i32,
    pub metadata: String,
    pub identity: i16,
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct GetBook {
    pub uid: i32,
    pub authorid: i32,
    pub bookid: i32,
    pub parentid: Option<i32>,
    pub title: String,
    pub body: String,
    pub identity: i16,
    pub metadata: String
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct GetBookNodesForPage {
    pub uid: i32,
    pub bookid: i32,
    pub pageid: i32,
    pub parentid: Option<i32>,
    pub title: String,
    pub body: String,
    pub identity: i16,
    pub metadata: String
}

#[derive(Deserialize)]
pub struct UpdateNode {
    pub tuid: i32,
    pub buid: i32,
}
#[derive(Deserialize)]
pub struct DeleteNode {
    pub docid: i32,
    pub nodes: Vec<i32>,
    pub update: Option<UpdateNode>,
}