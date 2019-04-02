#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub version: &'static str
}

