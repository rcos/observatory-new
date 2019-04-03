use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::models::User;
use crate::ObservDbConn;

pub struct UserGuard(User);

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = GuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut cookies = request.cookies();
        match cookies.get_private("user_id") {
            Some(uid) => {
                use crate::schema::users::dsl::*;
                let conn = request.guard::<ObservDbConn>().unwrap();
                match users.find(uid.value().parse::<i32>().unwrap()).first(&conn.0) {
                    Ok(u) => Outcome::Success(Self(u)),
                    Err(e) => {
                        Outcome::Failure((Status::InternalServerError, GuardError::DatabaseError(e)))
                    }
                }
            }
            None => Outcome::Failure((Status::Unauthorized, GuardError::NotLoggedIn))
        }
    }
}

pub struct MentorGuard(User);

impl<'a, 'r> FromRequest<'a, 'r> for MentorGuard {
    type Error = GuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let u = request.guard::<UserGuard>()?;
        // 0 is normal user so greater than is mentors and admins
        if u.0.tier > 0 {
            Outcome::Success(Self(u.0))
        } else {
            Outcome::Failure((Status::Forbidden, GuardError::NotMentor))
        }
    }
}

pub struct AdminGuard(User);

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = GuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let u = request.guard::<UserGuard>()?;
        // 1 is mentors so greater than is admins
        if u.0.tier > 1 {
            Outcome::Success(Self(u.0))
        } else {
            Outcome::Failure((Status::Forbidden, GuardError::NotMentor))
        }
    }
}

#[derive(Debug)]
pub enum GuardError {
    NotLoggedIn,
    NotMentor,
    NotAdmin,
    DatabaseError(diesel::result::Error),
}
