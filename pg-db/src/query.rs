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

/* Book */

pub static BOOK_DATA: &str = "SELECT uid, authorid, docid, parentid, title, body, identity, metadata FROM booknode WHERE docid=$1";

pub static UPDATE_BOOKS: &str = "UPDATE books SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

pub static DELETE_BOOKS: &str = "DELETE FROM books where uid=$1";
/* Book */


/* Blog */

pub static BLOG_DATA: &str = "SELECT uid, authorid, docid, parentid, title, body, identity, metadata FROM blognode WHERE docid=$1";

pub static CREATE_BLOG: &str = "INSERT INTO blog (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING blogid";

pub static CREATE_BLOG_NODE: &str = "INSERT INTO blognode (
    authorid, docid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

pub static UPDATE_BLOGS: &str = "UPDATE blogs SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

pub static DELETE_BLOGS: &str = "DELETE FROM blogs where uid=$1";

/* Blog */


pub static SIGNUP: &str = "INSERT INTO users (fname, lname, email, password) VALUES ($1, $2, $3, $4) RETURNING userid";