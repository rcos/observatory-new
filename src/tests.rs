use super::*;
use crate::auth::crypto::*;
use crate::models::*;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::local::Client;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Embed;

// Get all handlers in case we need to test other functions
pub use crate::attend::handlers::*;
pub use crate::auth::handlers::*;
pub use crate::calendar::handlers::*;
pub use crate::groups::handlers::*;
pub use crate::news::handlers::*;
pub use crate::projects::handlers::*;
pub use crate::users::handlers::*;

// Embed the Migrations into the binary
embed_migrations!("migrations/sqlite");

#[test]
fn launch() {
    let _client = Client::new(rocket()).unwrap();
    let response = _client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn check_static_content() {
    let _client = Client::new(rocket()).unwrap();
    let mut response = _client.get("/").dispatch();
    assert!(response.body().is_some());
    let body_str = response.body_string().unwrap();
    assert!(body_str.contains("chat.rcos.io"));
    Embed::get("img/favicon.webp").unwrap();
}

#[test]
fn add_user() {
    let mut db_exists = false;
    if Path::new("./observ.sqlite").is_file() {
        fs::rename("./observ.sqlite", "./observ.sqlite.backup")
            .ok()
            .expect("File Renaming Error");
        db_exists = true;
    }

    fs::File::create("./observ.sqlite")
        .ok()
        .expect("File Creation Error");

    let _client = Client::new(rocket()).unwrap();
    let conn_url = _client
        .rocket()
        .config()
        .get_table("databases")
        .unwrap()
        .get("sqlite_observ")
        .unwrap()
        .get("url")
        .unwrap()
        .as_str()
        .unwrap();

    let conn = SqliteConnection::establish(conn_url)
        .expect("Failed to connect to database in AddUserTest");
    embedded_migrations::run(&conn).expect("Failed to run embedded migrations");

    use crate::schema::users::dsl::*;
    let pass = String::from("thisisapassword");
    let psalt = gen_salt();
    let phash = hash_password(pass, &psalt);

    let nu = NewUser {
        real_name: String::from("John Doe"),
        handle: String::from("JD1"),
        password_hash: phash,
        salt: psalt,
        bio: String::from("This is a test user. Do not disturb."),
        email: String::from("doej@test-rcos.io"),
        tier: 0,
        active: true,
        mmost: String::from("JDMM"),
    };

    insert_into(users)
        .values(&nu)
        .execute(&conn)
        .expect("Failed to add user to database");

    let user: User = users
        .filter(&email.eq(&*nu.email))
        .first(&conn)
        .expect("Failed to get user from database");
    {
        use crate::schema::relation_group_user::dsl::*;
        insert_into(relation_group_user)
            .values(&NewRelationGroupUser {
                group_id: 0,
                user_id: user.id,
            })
            .execute(&conn)
            .expect("Failed to insert new relation into database");
    }

    assert_eq!("JD1".to_string(), user.handle);
    fs::remove_file("./observ.sqlite")
        .ok()
        .expect("File Deletion Error");

    if db_exists {
        fs::rename("./observ.sqlite.backup", "./observ.sqlite")
            .ok()
            .expect("File Renaming Error");
    }
}
