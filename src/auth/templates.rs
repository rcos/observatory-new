#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

#[derive(Template)]
#[template(path = "auth/signup.html")]
pub struct SignUpTemplate {
    pub logged_in: OptUser,
}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LogInTemplate {
    pub logged_in: OptUser,
}
