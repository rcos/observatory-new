use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies, RawStr};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::models::*;
use crate::templates::*;
use crate::ObservDbConn;

#[get("/")]
pub fn index() -> Index {
    Index {
        version: env!("CARGO_PKG_VERSION"),
    }
}

use rocket::response::NamedFile;
use std::path::PathBuf;

#[get("/static/<file..>")]
pub fn staticfile(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("static/").join(file)).ok()
}

#[get("/signup")]
pub fn signup() -> SignUp {
    SignUp
}

#[post("/signup", data = "<user>")]
pub fn signup_post(conn: ObservDbConn, mut cookies: Cookies, user: Form<NewUser>) -> Redirect {
    use crate::schema::users::dsl::*;

    let mut user = user.into_inner();
    user.password_hash = String::from("Password Hashing Not Implemented");

    insert_into(users)
        .values(&user)
        .execute(&conn.0)
        .expect("Failed to add user to database");

    let uid: i32 = users
        .filter(&email.eq(user.email))
        .first::<User>(&conn.0)
        .expect("Failed to get user from database")
        .id;

    cookies.add_private(Cookie::new("user_id", format!("{}", uid)));

    Redirect::to("/")
}

#[get("/users?<s>")]
pub fn users(conn: ObservDbConn, s: Option<&RawStr>) -> Users {
    Users {
        users: filter_users(&conn.0, s),
    }
}

#[get("/users.json?<s>")]
pub fn users_json(conn: ObservDbConn, s: Option<&RawStr>) -> Json<Vec<User>> {
    Json(filter_users(&conn.0, s))
}

fn filter_users(conn: &SqliteConnection, term: Option<&RawStr>) -> Vec<User> {
    use crate::schema::users::dsl::*;

    if let Some(term) = term {
        let sterm = format!("%{}%", term);
        let filter = real_name
            .like(&sterm)
            .or(email.like(&sterm))
            .or(handle.like(&sterm));
        users.filter(filter).load(conn)
    } else {
        users.load(conn)
    }
    .expect("Failed to get users")
}
