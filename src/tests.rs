use super::*;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn launch() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn add_user() {
    
}