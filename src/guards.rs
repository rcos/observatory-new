use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::models::User;
use crate::ObservDbConn;

pub type MaybeLoggedIn = Option<UserGuard>;

pub struct UserGuard(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = GuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut cookies = request.cookies();
        match cookies.get_private("user_id") {
            Some(uid) => {
                use crate::schema::users::dsl::*;
                let conn = request.guard::<ObservDbConn>().unwrap();
                match users
                    .find(uid.value().parse::<i32>().unwrap())
                    .first(&*conn)
                {
                    Ok(u) => Outcome::Success(Self(u)),
                    Err(e) => Outcome::Failure((
                        Status::InternalServerError,
                        GuardError::DatabaseError(e),
                    )),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, GuardError::NotLoggedIn)),
        }
    }
}

impl UserThroughOption for Option<UserGuard> {
    fn user(self) -> Option<User> {
        self.and_then(|u| Some(u.0))
    }
}

pub struct MentorGuard(pub User);

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

impl UserThroughOption for Option<MentorGuard> {
    fn user(self) -> Option<User> {
        self.and_then(|u| Some(u.0))
    }
}

pub struct AdminGuard(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = GuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let u = request.guard::<UserGuard>()?;
        // 1 is mentors so greater than is admins
        if u.0.tier > 1 {
            Outcome::Success(Self(u.0))
        } else {
            Outcome::Failure((Status::Forbidden, GuardError::NotAdmin))
        }
    }
}

impl UserThroughOption for Option<AdminGuard> {
    fn user(self) -> Option<User> {
        self.and_then(|u| Some(u.0))
    }
}

#[derive(Debug)]
pub enum GuardError {
    NotLoggedIn,
    NotMentor,
    NotAdmin,
    DatabaseError(diesel::result::Error),
}

pub trait UserThroughOption {
    fn user(self) -> Option<User>;
}
