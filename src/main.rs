// Needed by Rocket
#![feature(proc_macro_hygiene, decl_macro)]

// Ensure all the macros are imported
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate askama;
#[macro_use]
extern crate serde_derive;

use rocket::fairing::AdHoc;

// Module files
mod handlers;
mod helpers;
mod models;
mod schema;
mod templates;

use handlers::*;

// Central DB connection
#[database("sqlite_observ")]
pub struct ObservDbConn(diesel::SqliteConnection);

pub struct SecretKey(String);

fn main() {
    rocket::ignite()
        .attach(ObservDbConn::fairing())
        .attach(AdHoc::on_launch(|rocket| {
            let secret = rocket
                .config()
                .get_str("secret_key")
                .expect("Failed to get secret_key");
            Ok(rocket.manage(SecretKey(String::from(secret))))
        }))
        .mount(
            "/",
            routes![
                index,
                signup,
                signup_post,
                login,
                login_post,
                staticfile,
                user,
                users,
                users_json,
                project
            ],
        )
        .launch();
}
