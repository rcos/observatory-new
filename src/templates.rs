use crate::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub version: &'static str,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUp;

#[derive(Template)]
#[template(path = "users.html")]
pub struct Users {
    pub users: Vec<models::User>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LogIn;
