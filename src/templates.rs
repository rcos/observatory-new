use crate::users::models::User;

pub type OptUser = Option<User>;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: OptUser,
    pub version: &'static str,
}

use crate::users::models::GradeSummary;
use crate::groups::models::Group;
use crate::projects::models::Project;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub logged_in: OptUser,
    pub projects: Vec<Project>,
    pub groups: Vec<Group>,
    pub summary: GradeSummary
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

pub mod filters {
    pub use askama_filters::filters::*;
}
