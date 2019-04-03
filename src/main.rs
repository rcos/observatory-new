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
#[macro_use]
extern crate rust_embed;

// Module files
mod guards;
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
        .register(catchers![login_catch])
        .mount(
            "/",
            routes![
                index,
                calendar,
                newevent,
                newevent_post,
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
