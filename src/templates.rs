use crate::models::*;

type OptUser = Option<User>;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: OptUser,
    pub version: &'static str,
}

//# Sign Up and Log In templates

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUpTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LogInTemplate;

//# User Templates

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

//# Project Templates

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

//# Group Templates

#[derive(Template)]
#[template(path = "group.html")]
pub struct GroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
}

//# Calendar Templates

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

//# Catcher Templates

#[derive(Template)]
#[template(path = "catchers/403.html")]
pub struct Error403Template {
    pub logged_in: OptUser
}

#[derive(Template)]
#[template(path = "catchers/404.html")]
pub struct Error404Template {
    pub logged_in: OptUser
}
