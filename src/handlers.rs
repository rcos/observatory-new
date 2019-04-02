use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::helpers::*;
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
pub fn signup_post(
    conn: ObservDbConn,
    mut cookies: Cookies,
    user: Form<NewUser>,
) -> Redirect {
    use crate::schema::users::dsl::*;

    let mut user = user.into_inner();
    let newsalt = gen_salt();
    user.salt = newsalt.clone();
    user.password_hash = hash_password(user.password_hash, &newsalt);

    insert_into(users)
        .values(&user)
        .execute(&conn.0)
        .expect("Failed to add user to database");

    let user: User = users
        .filter(&email.eq(user.email))
        .first::<User>(&conn.0)
        .expect("Failed to get user from database");

    cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));

    Redirect::to(format!("/user/{}", user.handle))
}

#[get("/login")]
pub fn login() -> LogIn {
    LogIn
}

#[post("/login", data = "<creds>")]
pub fn login_post(
    conn: ObservDbConn,
    mut cookies: Cookies,
    creds: Form<LogInForm>,
) -> Redirect {
    use crate::schema::users::dsl::*;

    let creds = creds.into_inner();

    let user: User = users
        .filter(&email.eq(creds.email))
        .first(&conn.0)
        .expect("Failed to get user from database");

    if verify_password(creds.password, user.password_hash, &user.salt) {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

#[get("/user/<h>")]
pub fn user(conn: ObservDbConn, h: String) -> User {
    use crate::schema::users::dsl::*;

    users
        .filter(handle.like(h))
        .first(&conn.0)
        .expect("Failed to get user from database")
}

#[get("/users?<s>")]
pub fn users(conn: ObservDbConn, s: Option<String>) -> Users {
    Users {
        users: filter_users(&conn.0, s),
    }
}

#[get("/users.json?<s>")]
pub fn users_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<User>> {
    Json(filter_users(&conn.0, s))
}

#[get("/projects?<s>")]
pub fn projects(conn: ObservDbConn, s: Option<String>) -> Projects {
    Projects { 
        projects: filter_projects(&conn.0, s)
     }
}

#[get("/project/<n>")]
pub fn project(conn: ObservDbConn, n: String) -> Project {
    use crate::schema::projects::dsl::*;

    projects
        .filter(name.like(n))
        .first(&conn.0)
        .expect("Failed to get project from database")
}
