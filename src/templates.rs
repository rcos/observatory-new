//! Misc HTML templates
//!
//! Various templates that do not belong to any of the other modules or
//! are "top-level" such as the index and static route.

use crate::models::User;

/// Companion to `MaybeLoggedIn`
///
/// This is a simple wrapper to act as the companion to `MaybeLoggedIn`
/// where that is a Guard and this is just the `User`.
pub type OptUser = Option<User>;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: OptUser,
    pub version: &'static str,
}
 
/// Big text template
/// 
/// This is a simple template that just shows the given text large
/// across the screen. Useful for attendance codes.
#[derive(Template)]
#[template(path = "big.html")]
pub struct BigTemplate {
    pub logged_in: OptUser,
    pub text: String
}

use crate::models::GradeSummary;
use crate::models::Group;
use crate::models::Project;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub logged_in: OptUser,
    pub projects: Vec<Project>,
    pub groups: Vec<Group>,
    pub summary: GradeSummary,
}

//# Catcher Templates

#[derive(Template)]
#[template(path = "catchers/403.html")]
pub struct Error403Template {
    pub logged_in: OptUser,
}

#[derive(Template)]
#[template(path = "catchers/404.html")]
pub struct Error404Template {
    pub logged_in: OptUser,
}

/// Puts the filters in the proper namespace
pub mod filters {
    pub use askama_filters::filters::*;
}

/// An error in an HTML form
///
/// This enum is used to represent errors in HTML forms.
/// Feedback is provided to the user using `form-error.html`.
/// Generally you pass this back as a GET argument to the form page.
#[derive(Debug)]
pub enum FormError {
    /// The email was incorrect or missing
    Email,
    /// The password was incorrect or missing
    Password,
    /// When you don't want to specify if it was an
    /// issue witht he email or the password
    Credentials,
    /// The password and it's repeat are not the same
    PasswordMismatch,
    /// Some other unknown error
    Other,
}

use std::fmt;

// Converts to a string
impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FormError::Email => "email",
                FormError::Password => "password",
                FormError::Credentials => "credentials",
                FormError::PasswordMismatch => "mismatch",
                FormError::Other => "other",
            }
        )
    }
}

// Converts from a string
impl<T: AsRef<str>> From<T> for FormError {
    fn from(f: T) -> FormError {
        match f.as_ref() {
            "email" => FormError::Email,
            "password" => FormError::Password,
            "credentials" => FormError::Credentials,
            "mismatch" => FormError::PasswordMismatch,
            "other" => FormError::Other,
            _ => FormError::Other,
        }
    }
}

use rocket::http::RawStr;
use rocket::request::FromFormValue;

impl<'v> FromFormValue<'v> for FormError {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormError, &'v RawStr> {
        Ok(FormError::from(form_value))
    }
}
