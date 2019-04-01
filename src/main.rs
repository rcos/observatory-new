// This is here for macro_use
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate gotham_derive;

use std::env;

use dotenv::dotenv;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;

mod handlers;
mod middleware;
mod models;
mod schema;

use handlers::*;

fn main() {
    dotenv().ok();

    let addr = env::var("SERVE_URL").unwrap_or(String::from("0.0.0.0:7878"));
    println!("Starting observatory at http://{}", addr);
    gotham::start(addr, router())
}
