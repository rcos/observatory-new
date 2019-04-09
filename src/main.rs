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

extern crate rand;

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
        .register(catchers![catch_401, catch_403, catch_404])
        .mount(
            "/",
            routes![
                index,
                staticfile,
                // Calendar
                calendar,
                calendar_json,
                event,
                newevent,
                newevent_post,
                // Sign Up and Log In
                signup,
                signup_post,
                login,
                login_post,
                logout,
                // Attendance
                attend,
                attend_post,
                // Users
                user,
                user_by_id,
                users,
                users_json,
                // Projects
                project,
                project_by_id,
                projects,
                projects_json,
                // Groups
                group,
                groups,
                newgroup_post,
                newmeeting_post
            ],
        )
        .launch();
}
