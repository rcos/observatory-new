#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub version: &'static str
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUp;