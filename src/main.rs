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
        .attach(AdminCheck)
        .register(catchers![catch_401, catch_403, catch_404])
        .mount(
            "/",
            routes![
                index,
                staticfile,
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
                // Projects
                project,
                project_by_handle,
                projects,
                projects_json,
                newproject,
                newproject_post,
                project_delete,
                // Groups
                group,
                groups,
                newgroup_post,
                newmeeting_post,
                // News
                news,
                news_json,
                newsstory,
                newsstory_delete
            ],
        )
        .launch();
}

// Checks if the Admin user has a password
// and generates one if it doesn't
pub struct AdminCheck;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

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

        use crate::models::{NewUser, User};
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        use diesel::sqlite::SqliteConnection;

        let conn = SqliteConnection::establish(conn_url)
            .expect("Failed to connect to database in AdminCheck");

        let admin: User = users
            .find(0)
            .first(&conn)
            .expect("Failed to get admin from database");

        if admin.password_hash.is_empty() {
            use crate::helpers::*;

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
