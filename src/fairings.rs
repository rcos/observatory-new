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

/// Check for the config file at attach
///
/// If there is no config file specified Rocket is going to fallback to defaults.
/// This writes those defaults to a file and makes some alterations
/// to be what Observatory wants/expects.
pub struct ConfigWrite;

impl Fairing for ConfigWrite {
    fn info(&self) -> Info {
        Info {
            name: "Writing config if it does not exist",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> std::result::Result<Rocket, Rocket> {
        use rocket::config;
        // Get the current config
        let mut conf = rocket.config().clone();

        // If there is a config file just return
        if conf.root().is_some() {
            return Ok(rocket);
        }

        println!("\tRocket.toml does not exist, generating...");
        println!(
            "\tWARNING: If you set your secret_key via enviroment variable it will be overwritten!"
        );

        conf.set_root(".");

        // If the database is not set
        if conf.get_extra("databases").is_err() {
            // Pick the DB url based on the environment
            let dburl = match conf.environment {
                config::Environment::Development | config::Environment::Staging => {
                    String::from("./observ.sqlite")
                }
                config::Environment::Production => {
                    use std::path::Path;
                    let p = Path::new("/var/lib/observatory/");
                    // If we can write to the default production location do so
                    if p.exists() && !p.metadata().unwrap().permissions().readonly() {
                        format!("{}{}", p.to_str().unwrap(), "observ.sqlite")
                    } else {
                        String::from("./observ.sqlite")
                    }
                }
            };

            // The really ugly but necessary way we need to set the DB url
            use std::collections::HashMap;
            let mut hash = HashMap::<String, config::Value>::new();
            hash.insert(String::from("databases"), {
                let mut h = HashMap::new();
                h.insert(String::from("sqlite_observ"), {
                    let mut h = HashMap::new();
                    h.insert(String::from("url"), dburl);
                    h
                });
                config::Value::try_from(h).unwrap()
            });
            conf.set_extras(hash);
        }

        // If in production mode generate and set a secret key
        let s = if rocket.config().environment.is_prod() {
            // Generate a new secret key
            let s = gen_secret();
            // Set the key in the config
            conf.set_secret_key(s.clone()).unwrap();
            s
        } else {
            String::new()
        };
        // Write the config with the key to a file
        write_config(&conf, &s).unwrap();

        // Write the config to a file
        // Return the new Rocket based on the modified config
        Ok(rocket::custom(conf))
    }
}

/// Generates a new secret key using Ring
fn gen_secret() -> String {
    use base64::encode;
    use ring::rand::{SecureRandom, SystemRandom};

    let mut buf: [u8; 32] = [0; 32];
    SystemRandom::new().fill(&mut buf).unwrap();
    encode(&buf)
}

/// Writes the config to a file in the same folder as the binary
fn write_config(conf: &rocket::Config, secret: &String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    // Format the string that is going to be written
    let mut outstring = format!(
        "[{}]\naddress = \"{}\"\nport = {}\nlog = \"{}\"\n",
        conf.environment, conf.address, conf.port, conf.log_level
    );

    // Don't do a secret key if none is given
    if !secret.is_empty() {
        outstring += &format!("secret_key = \"{}\"\n", secret);
    }

    // Get the extra fields like the databases and add them
    outstring += &format!(
        "databases = {{ sqlite_observ = {{ url = {} }} }}",
        conf.extras
            .get("databases")
            .unwrap()
            .get("sqlite_observ")
            .unwrap()
            .get("url")
            .unwrap()
            .as_str()
            .unwrap()
    );

    File::create("./Rocket.toml")?.write_all(outstring.as_bytes())
}
