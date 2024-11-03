#[macro_use] extern crate rocket;

pub mod blocks;
pub mod transactions;
pub mod addresses;

use rocket::routes;

pub fn create_routes() -> Vec<rocket::Route> {
    routes![
        blocks::get_blocks,
        transactions::get_transactions,
        addresses::get_address
    ]
}
