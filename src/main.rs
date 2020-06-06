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
extern crate flexi_logger;
extern crate log;
#[macro_use]
extern crate diesel_derive_newtype;

#[macro_use]
mod macros {
    /// A simple macro for logging audit messages
    #[macro_export]
    macro_rules! audit_log {
        ($($arg:tt)*) => (
            $crate::info!(target: "{Audit}", "{}", format_args!($($arg)*));
        )
    }
}

// Module files
mod fairings;
mod guards;
mod handlers;
mod schema;
mod templates;
#[cfg(test)]
mod tests;

// Table Modules
mod attend;
mod auth;
mod calendar;
mod groups;
mod news;
mod projects;
mod users;

use flexi_logger::{opt_format, writers::FileLogWriter, LogTarget, Logger};
use log::*;

const LOG_DIR: &str = "logs";

/// The database connection
///
/// This struct is the wrapper for the database connection which
/// is mounted as a fairing and can be accessed as a request guard.
#[database("sqlite_observ")]
pub struct ObservDbConn(diesel::SqliteConnection);

fn audit_writer() -> Box<FileLogWriter> {
    Box::new(
        FileLogWriter::builder()
            .discriminant("audit")
            .suffix("log")
            .format(opt_format)
            .suppress_timestamp()
            .directory(LOG_DIR)
            .append()
            .print_message()
            .try_build()
            .unwrap(),
    )
}

pub fn rocket(test_config: Option<rocket::Config>) -> rocket::Rocket {
    // Load all the handlers
    use handlers::*;

    // Load the fairings
    use fairings::{AdminCheck, ConfigWrite, DatabaseCreate};

    let app = if let Some(test_config) = test_config {
        rocket::custom(test_config)
    } else {
        rocket::ignite()
    };

    // Prepare webserver
    app.attach(ConfigWrite)
        // Attach fairings
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
                big,
                staticfile,
                favicon,
                dashboard,
                sitemap,
                // Calendar
                calendar,
                calendar_json,
                calendar_ics,
                event,
                event_edit,
                event_edit_put,
                event_delete,
                event_new,
                event_new_post,
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
                user_edit,
                user_edit_put,
                user_delete,
                // Projects
                project,
                project_by_handle,
                projects,
                projects_json,
                project_new,
                project_new_post,
                project_delete,
                project_edit,
                project_edit_put,
                project_join,
                project_join_post,
                project_members,
                project_members_json,
                project_member_add,
                project_member_add_post,
                project_member_delete,
                // Groups
                group,
                groups,
                groups_json,
                group_new,
                group_new_post,
                group_user_add,
                group_user_add_post,
                group_user_delete,
                group_delete,
                meetings,
                meeting_get,
                meetings_json,
                meeting_new_post,
                group_edit,
                group_edit_put,
                // News
                news,
                news_json,
                news_rss,
                news_slides,
                story,
                story_new,
                story_new_post,
                story_delete,
                story_edit,
                story_edit_put,
            ],
        )
}

/// The main function that starts the program
///
/// This is the standard `main` function that acts as the start of the program.
/// Here it loads Rocket, sets it up with the fairings and handlers,
/// then launches the server.
fn main() {
    Logger::with_env_or_str("info")
        .print_message()
        .log_to_file()
        .directory(LOG_DIR)
        .add_writer("Audit", audit_writer())
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    audit_log!("Audit Logger Initialized!");

    // Liftoff! Starts the webserver
    rocket(None).launch();
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
        fn group_id(&self) -> Option<i32> {
            None
        }
        fn is_event(&self) -> bool;
        fn url(&self) -> String;
    }
}
