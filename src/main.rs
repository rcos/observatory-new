// Needed by Rocket
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate askama;
#[macro_use]
extern crate diesel;
extern crate rand;
// Ensure all the macros are imported
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_derive;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

use handlers::*;

// Module files
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

// Central DB connection
#[database("sqlite_observ")]
pub struct ObservDbConn(diesel::SqliteConnection);

fn main() {
    use crate::attend::handlers::*;
    use crate::auth::handlers::*;
    use crate::calendar::handlers::*;
    use crate::groups::handlers::*;
    use crate::news::handlers::*;
    use crate::projects::handlers::*;
    use crate::users::handlers::*;

    rocket::ignite()
        .attach(ObservDbConn::fairing())
        .attach(AdminCheck)
        .register(catchers![catch_401, catch_403, catch_404])
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
                group_delete,
                newmeeting_post,
                editgroup,
                editgroup_put,
                // News
                news,
                news_json,
                news_rss,
                newsstory,
                newnewsstory,
                newnewsstory_post,
                newsstory_delete,
                editnewsstory,
                editnewsstory_put,
            ],
        )
        .launch();
}

// Checks if the Admin user has a password
// and generates one if it doesn't
pub struct AdminCheck;

impl Fairing for AdminCheck {
    fn info(&self) -> Info {
        Info {
            name: "Admin Password Check",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        // Get the database url from the config
        let conn_url = rocket
            .config()
            .get_table("databases")
            .unwrap()
            .get("sqlite_observ")
            .unwrap()
            .get("url")
            .unwrap()
            .as_str()
            .unwrap();

        use crate::schema::users::dsl::*;
        use crate::users::models::{NewUser, User};
        use diesel::prelude::*;
        use diesel::sqlite::SqliteConnection;

        let conn = SqliteConnection::establish(conn_url)
            .expect("Failed to connect to database in AdminCheck");

        let admin: User = users
            .find(0)
            .first(&conn)
            .expect("Failed to get admin from database");

        if admin.password_hash.is_empty() {
            use crate::attend::code::gen_code;
            use crate::auth::crypto::*;

            let pass = gen_code();
            eprintln!(
                "\tADMIN PASSSWORD: {}\n\tCHANGE THIS AS SOON AS POSSIBLE",
                pass
            );

            let psalt = gen_salt();
            let phash = hash_password(pass, &psalt);

            // Needs to be a NewUser for set()
            let nu = NewUser {
                real_name: admin.real_name,
                handle: admin.handle,
                password_hash: phash,
                salt: psalt,
                bio: admin.bio,
                email: admin.email,
                tier: admin.tier,
                active: admin.active,
            };

            use diesel::update;
            update(users.find(0))
                .set(&nu)
                .execute(&conn)
                .expect("Failed to update admin user in database");
        }
    }
}

pub mod models {
    use chrono::NaiveDateTime;
    use std::fmt::{Debug};

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
