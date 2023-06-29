mod db_connect;
use mysql::*;
use mysql::prelude::*;
use db_connect::connect;

#[derive(Debug)]
pub struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

pub fn select(){
    let mut conn = connect();
    let selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(customer_id, amount, account_name)| {
                Payment { customer_id, amount, account_name }
            },
        ).unwrap();
    println!("{:#?}", selected_payments);
}
