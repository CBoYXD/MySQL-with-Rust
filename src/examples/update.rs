use crate::examples::db::connect::connect;
use mysql::{prelude::Queryable, params};


pub fn update_user(user_id: i32, new_name: &str) -> () {
    let mut conn = connect();
    let query = "UPDATE user SET user_fullname = :user_fullname WHERE user_id = :user_id";
    conn.exec_drop(query, params!{
        "user_fullname" => new_name,
        "user_id" => user_id
    }).unwrap();

    println!("Update table");
}