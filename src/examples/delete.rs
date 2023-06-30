use crate::examples::db::connect::connect;
use mysql::{*, prelude::Queryable};


pub fn delete_data(user_id: i32) -> (){
    let mut conn = connect();
    let query = "DELETE FROM user WHERE user_id = :user_id";
    conn.exec_drop(query, params! {
        "user_id" => user_id
    }).unwrap();
    println!("Successfully deleted user")
}