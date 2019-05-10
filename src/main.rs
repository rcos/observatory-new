//! The Rust rewrite of the [RCOS](https://rcos.io) website.
//! This version is intended to be more maintainable in the long term
//! and uses a simpler and more conservative set of tools without being
//! entirely ["boring software"](https://tqdev.com/2018-the-boring-software-manifesto).
//!
//! ## Project Structure
//! Each folder, or "module", in this project corresponds to a different
//! logical part of Observatory. Most modules contain the following files:
//!
//! - `mod.rs` Declares the folder as a module and exports its modules.
//! - `handlers.rs` HTTP handlers for each route. The core logic of Observatory
//! - `models.rs` Database models used to fetch and insert into the DB.
//! - `templates.rs` Defines the state and types for the HTML templates.

// Needed by Rocket
#![feature(proc_macro_hygiene, decl_macro)]

// Ensure all the macros are imported
#[macro_use]
extern crate askama;
#[macro_use]
extern crate diesel;
#[doc(hidden)]
#[macro_use]
extern crate rocket;
#[doc(hidden)]
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel_migrations;

// Module files
mod fairings;
mod guards;
mod handlers;
mod schema;
mod templates;

// Table Modules
mod attend;
mod auth;
mod calendar;
mod groups;
mod news;
mod projects;
mod users;

#[database("sqlite_observ")]
pub struct ObservDbConn(diesel::SqliteConnection);

/// The main function that starts the program
///
/// This is the standard `main` function that acts as the start of the program.
/// Here it loads Rocket, sets it up with the fairings and handlers,
/// then launches the server.
fn main() {
    // Load all the handlers
    use handlers::*;

    // Load the fairings
    use fairings::{AdminCheck, ConfigWrite, DatabaseCreate};

    // Prepare webserver
    rocket::ignite()
        // Attach fairings
        .attach(ConfigWrite)
        .attach(DatabaseCreate)
        .attach(AdminCheck)
        .attach(ObservDbConn::fairing())
        // Register Catchers
        .register(catchers![catch_401, catch_403, catch_404])
        // Mount handlers
        .mount(
            "/",
            routes![
                index,
                staticfile,
                favicon,
                dashboard,
                // Calendar
                calendar,
                calendar_json,
                event,
                editevent,
                editevent_put,
                event_delete,
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
                user_by_handle,
                users,
                users_json,
                edituser,
                edituser_put,
                user_delete,
                // Projects
                project,
                project_by_handle,
                projects,
                projects_json,
                newproject,
                newproject_post,
                project_delete,
                editproject,
                editproject_put,
                join,
                join_post,
                // Groups
                group,
                groups,
                newgroup,
                newgroup_post,
                adduser,
                adduser_post,
                removeuser,
                group_delete,
                newmeeting_post,
                editgroup,
                editgroup_put,
                // News
                news,
                news_json,
                news_rss,
                news_slides,
                newsstory,
                newnewsstory,
                newnewsstory_post,
                newsstory_delete,
                editnewsstory,
                editnewsstory_put,
            ],
        )
        // Liftoff! Starts the webserver
        .launch();
}

/// Top-level module containing all the models.
///
/// This mostly just re-exports the models from their
/// respective modules to provide an easy way to import.
pub mod models {
    use chrono::NaiveDateTime;
    use std::fmt::Debug;

    // Import then re-export all models
    pub use crate::attend::models::*;
    pub use crate::calendar::models::*;
    pub use crate::groups::models::*;
    pub use crate::news::models::*;
    pub use crate::projects::models::*;
    pub use crate::users::models::*;

    /// Represents anything that can be attended such as meetings and events.
    ///
    /// Used to create generics accross anything attendable.
    /// The trait mostly just defines functions to access the fields of the
    /// structs that are Attendable.
    pub trait Attendable: Debug {
        fn id(&self) -> i32;
        fn name(&self) -> String;
        fn time(&self) -> NaiveDateTime;
        fn code(&self) -> String;
        fn owner_id(&self) -> i32;
        fn is_event(&self) -> bool;
        fn url(&self) -> String;
    }
}
