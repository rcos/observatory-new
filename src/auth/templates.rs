//! HTML templates for login and signup

#[allow(unused_imports)]
use crate::templates::{filters, FormError, OptUser};

/// Sign Up page template
///
/// HTML File: `auth/signup.html`
///
/// Page that shows the sign up form
#[derive(Template)]
#[template(path = "auth/signup.html")]
pub struct SignUpTemplate {
    pub logged_in: OptUser,
    pub error: Option<FormError>,
}

/// Log In page template
///
/// HTML File: `auth/login.html`
///
/// Page that shows the log in form
#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LogInTemplate {
    pub logged_in: OptUser,
    pub error: Option<FormError>,
}
