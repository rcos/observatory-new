// Needed by Rocket
#![feature(proc_macro_hygiene, decl_macro)]

// This is here for macro_use
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate askama;

use std::env;
use std::path::PathBuf;

use rocket::response::NamedFile;

mod models;
mod schema;
mod templates;

use templates::*;

#[database("sqlite_observ")]
struct ObservDbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Index {
    Index {
        version: env!("CARGO_PKG_VERSION")
    }
}

#[get("/static/<file..>")]
fn staticfile(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("static/").join(file)).ok()
}

fn main() {
    let addr = env::var("SERVE_URL").unwrap_or(String::from("localhost:8000"));
    println!("Starting observatory at http://{}", addr);

    rocket::ignite()
        .attach(ObservDbConn::fairing())
        .mount("/", routes![index, staticfile]).launch();
}
