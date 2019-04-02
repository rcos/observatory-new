use rocket::request::Form;
use rocket::response::Redirect;
use diesel::insert_into;
use diesel::RunQueryDsl;

use crate::templates::*;
use crate::models::*;
use crate::ObservDbConn;

#[get("/")]
pub fn index() -> Index {
    Index {
        version: env!("CARGO_PKG_VERSION")
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

#[post("/signup", data="<user>")]
pub fn signup_post(conn: ObservDbConn, user: Form<NewUser>) -> Redirect {
    use crate::schema::users::dsl::*;

    insert_into(users)
        .values(&user.into_inner())
        .execute(&conn.0)
        .expect("Failed to add user to database");

    Redirect::to("/")
}