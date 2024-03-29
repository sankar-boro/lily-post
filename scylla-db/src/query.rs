#![allow(unused_imports, dead_code)]

#[macro_export]
macro_rules! create_query {
    ( $a:expr, $( $x:expr ),* ) => {
        {
            let xx = format!("INSERT INTO {}", $a);
            let mut aa = String::from("(");
            let mut bb = String::from("VALUES (");
            $(
                aa.push_str($x);
                aa.push_str(", ");
                bb.push_str("?, ");
            )*
            let mut aa = format!("{}", &aa[0..aa.len()-2]);
            let mut bb = format!("{}", &bb[0..bb.len()-2]);
            aa.push_str(")");
            bb.push_str(")");
            format!("{} {} {}", xx, aa, bb)
        }
    };
}

pub static CREATE_BOOK_NODE_QUERY: &str = "INSERT INTO sankar.book (
    docid, pageId, uniqueId, parentId, authorId, title, body, metadata, url, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BLOG_NODE_QUERY: &str = "INSERT INTO sankar.blog (
    docid, uniqueId, parentId, authorId, title, body, metadata, url, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BOOKS: &str = "INSERT INTO sankar.books (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
/**
 * We dont include parentId, because the first node is the parent node.
 */
pub static CREATE_BOOK: &str = "INSERT INTO sankar.book (
    docid, pageId, uniqueId, authorId, title, body, url, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_BOOK_TITLE: &str = "INSERT INTO sankar.book_title (
    docid, parentId, uniqueId, title, identity
) VALUES(
    ?, ?, ?, ?, ?
)";
pub static CREATE_USER_BOOKS: &str = "INSERT INTO sankar.userbooks (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_CATEGORY_BOOKS: &str = "INSERT INTO sankar.categorybooks (
    category, docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static ADD_USER_CATEGORY: &str = "INSERT INTO sankar.usercategories (
    authorId, category, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static DELETE_CATEGORY: &str = "DELETE FROM sankar.usercategories WHERE authorId=? AND category=?";
pub static CREATE_BLOGS: &str = "INSERT INTO sankar.blogs (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_BLOG: &str = "INSERT INTO sankar.blog (
    docid, uniqueId, authorId, title, body, url, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_USER_BLOGS: &str = "INSERT INTO sankar.userblogs (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_CATEGORY_BLOGS: &str = "INSERT INTO sankar.categoryblogs (
    category, docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_ALLCATEGORY: &str = "INSERT INTO sankar.allcategories (
    category, authorId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";

pub static CREATE_USER_BOOK_SETTINGS: &str = "INSERT INTO sankar.userbooksettings (
    authorId, docid, settings
) VALUES(
    ?, ?, ?
)";

pub static UPDATE_USER_BOOK_SETTINGS: &str = "UPDATE sankar.userbooksettings SET settings=? where authorId=? AND docid=?";

pub static FOLLOW_USER: &str = "INSERT INTO sankar.followers (
    userId, followerId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static UNFOLLOW_USER: &str = "DELETE FROM sankar.followers WHERE userId=? AND followerId=?";