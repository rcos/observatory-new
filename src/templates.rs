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

#[derive(Template)]
#[template(path = "projects.html")]
pub struct Projects {
    pub projects: Vec<models::Project>
}

#[derive(Template)]
#[template(path = "calendar.html")]
pub struct Calendar {
    pub events: Vec<models::Event>
}