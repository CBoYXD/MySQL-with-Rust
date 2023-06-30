// Get connect public! function from src/examples/db/connect.rs
use crate::examples::db::connect::connect;
use mysql::{*, prelude::Queryable};

/* struct User{
    user_id: i32,
    user_fullname: String,
    have_username: bool,
    lang: String
} */


// Based on User struct, we will create the User table


// Create public function create_table
pub fn create_table() -> () {
    // Get connect in variable conn
    let mut conn: PooledConn = connect();
    // Request to db
    conn.query_drop(
        r"CREATE TABLE User (
            user_id int not null,
            user_fullname text not null,
            have_username bool not null,
            lang text not null
        )").unwrap();
    
    print!("Table is created");
}