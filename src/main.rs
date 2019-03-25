// This is here for the macro_use
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

const FALLBACK_DB_URL: &str = "db.sqlite";

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!(
            "DATABASE_URL env variable not set, falling back to '{}'",
            FALLBACK_DB_URL
        );
        String::from(FALLBACK_DB_URL)
    });

    let db_conn = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to database at {}", database_url));
}
