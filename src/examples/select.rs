use crate::examples::db::connect::connect;
use mysql::{prelude::Queryable};

// Created struct User
#[derive(Debug)]
struct User{
    user_id: i32,
    user_fullname: String,
    have_username: bool,
    lang: String
}

pub fn select_data() -> (){
    let mut conn = connect();
    
    let get_users = conn.query_map(
        "SELECT user_id, user_fullname, have_username, lang from user",
        |(user_id, user_fullname, have_username, lang)| {
            User {user_id, user_fullname, have_username, lang}
        },
    ).unwrap();
    
    println!("{:#?}", get_users);

}