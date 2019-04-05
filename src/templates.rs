use crate::models::*;

type OptUser = Option<User>;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: OptUser,
    pub version: &'static str,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUpTemplate;

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub logged_in: OptUser,
    pub user: User,
}

#[derive(Template)]
#[template(path = "users-list.html")]
pub struct UsersListTemplate {
    pub logged_in: OptUser,
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LogInTemplate;

#[derive(Template)]
#[template(path = "project.html")]
pub struct ProjectTemplate {
    pub logged_in: OptUser,
    pub project: Project,
}

#[derive(Template)]
#[template(path = "projects-list.html")]
pub struct ProjectsListTemplate {
    pub logged_in: OptUser,
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "group.html")]
pub struct GroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
}

#[derive(Template)]
#[template(path = "calendar.html")]
pub struct CalendarTemplate {
    pub logged_in: OptUser,
    pub events: Vec<Event>,
}

#[derive(Template)]
#[template(path = "new-event.html")]
pub struct NewEventTemplate {
    pub logged_in: OptUser
}
