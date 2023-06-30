// Get connect function from src/examples/db/connect.rs
use crate::examples::db::connect::connect;
use mysql::{*, prelude::Queryable};

// Created struct User
struct User{
    user_id: i32,
    user_fullname: String,
    have_username: bool,
    lang: String
}


// Create public function insert_data
pub fn insert_data() -> (){
    let users: Vec<User> = vec![
    
    User {  user_id: 132132,
            user_fullname: "Alex".to_string(),
            have_username: true,
            lang: "English".to_string() }, 
    User {
            user_id: 135231,
            user_fullname: "Daniel".to_string(),
            have_username: false,
            lang: "Ukrainian".to_string()}
];
    // Get connect in variable conn
    let mut conn: PooledConn = connect();
    // Insert into table
    conn.exec_batch("INSERT INTO user (user_id, user_fullname, have_username, lang) VALUES (:user_id, :user_fullname, :have_username, :lang)",
    users.iter().map(|u| params! {
        "user_id" => &u.user_id,
        "user_fullname" => &u.user_fullname,
        "have_username" => &u.have_username,
        "lang" => &u.lang,
    }),).unwrap();            
    
    println!("All is Ok");

}