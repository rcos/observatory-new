//! Page guards
//!
//! These are all [Rocket request guards](https://rocket.rs/v0.4/guide/requests/#request-guards)
//! and are mostly used to validate that the user is logged in and has
//! permission to view the page they are trying to.

use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::models::User;
use crate::ObservDbConn;

/// A user might be logged in
///
/// This type wraps `UserGuard` and is used to represent that a user
/// might be logged in, or they might not and that it doesn't
/// matter which to view the page, but you still want to know they user's
/// info if they are logged in.
pub type MaybeLoggedIn = Option<UserGuard>;

/// Guards page for logged in Users
///
/// When using this guards and not `MaybeLoggedIn` the user *must* be
/// logged in to access the page.
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

/// Guards page for Mentors
///
/// The user must be logged in **and** be of the Mentor privledge tier (>0)
/// in order to access the page.
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

/// Guards page for Admins
///
/// The user must be logged in **and** be of the Admin privledge tier (>1)
/// in order to access the page.
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

/// Errors that guards can throw
///
/// The various errors that a guard can throw
#[derive(Debug)]
pub enum GuardError {
    NotLoggedIn,
    NotMentor,
    NotAdmin,
    DatabaseError(diesel::result::Error),
}

/// Access a user through an Option<Guard>
///
/// This trait defines a convience function to access a user
/// through a `Option<UserGuard>` or similar.
pub trait UserThroughOption {
    fn user(self) -> Option<User>;
}
