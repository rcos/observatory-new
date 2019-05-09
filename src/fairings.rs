//! Various middleware
//!
//! Read the [Rocket docs on Fairings](https://rocket.rs/v0.4/guide/fairings/)
//! for more information about how these work.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

// Embed the Migrations into the binary
embed_migrations!("migrations/sqlite");

/// Apply database migrations at launch
///
/// This will run at the launch of the webserver and apply any unapplied
/// database maigrations. This serves to create the database at launch so
/// that you don't have to manually.
pub struct DatabaseCreate;

impl Fairing for DatabaseCreate {
    fn info(&self) -> Info {
        Info {
            name: "Create Database if Needed",
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

        use diesel::prelude::*;
        let conn = SqliteConnection::establish(conn_url)
            .expect("Failed to connect to database in DatabaseCreate");

        // Run the embedded migrations
        embedded_migrations::run(&conn).expect("Failed to run embedded migrations");
    }
}

/// Check for admin password at launch
///
/// Checks if the Admin user has a password and generates one if it doesn't.
/// This password is printed to `stdout`.
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

        // Import needed things
        use crate::models::{NewUser, User};
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = SqliteConnection::establish(conn_url)
            .expect("Failed to connect to database in AdminCheck");

        let admin: User = users
            .find(0)
            .first(&conn)
            .expect("Failed to get admin from database");

        // Check if there is no password.
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

            // Needs to be a NewUser for set() so create it
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
