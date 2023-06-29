extern crate dotenv;
use std::env;
use dotenv::dotenv;
use mysql::*;


fn read_env() -> String{
    dotenv().expect("Failed to read env");
    let url = env::var("URL").expect("Failed to get url");
    return url;
}

pub fn connect() -> PooledConn{
    let url = read_env();
    let url = url.trim();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().expect("Failed to get connection");
    return conn
}