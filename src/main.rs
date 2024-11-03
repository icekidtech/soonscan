#[macro_use] extern crate rocket;

mod api;
mod db;
mod models;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", api::create_routes())
}
