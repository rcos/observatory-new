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

fn main() {
    rocket::ignite()
        .attach(ObservDbConn::fairing())
        .mount(
            "/",
            routes![
                index,
                calendar,
                signup,
                signup_post,
                login,
                login_post,
                logout,
                staticfile,
                user,
                users,
                users_json,
                project,
                projects
            ],
        )
        .launch();
}
