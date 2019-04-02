// Needed by Rocket
#![feature(proc_macro_hygiene, decl_macro)]

// Ensure all the macros are imported
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate askama;

// Module files
mod models;
mod schema;
mod templates;
mod handlers;

use handlers::*;

// Central DB connection
#[database("sqlite_observ")]
pub struct ObservDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(ObservDbConn::fairing())
        .mount("/", routes![index, signup, signup_post, staticfile]).launch();
}
