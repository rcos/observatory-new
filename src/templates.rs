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
#[template(path = "auth/signup.html")]
pub struct SignUpTemplate;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LogInTemplate;

//# User Templates

#[derive(Template)]
#[template(path = "user/user.html")]
pub struct UserTemplate {
    pub logged_in: OptUser,
    pub user: User,
}

#[derive(Template)]
#[template(path = "user/users-list.html")]
pub struct UsersListTemplate {
    pub logged_in: OptUser,
    pub users: Vec<User>,
}

//# Project Templates

#[derive(Template)]
#[template(path = "project/project.html")]
pub struct ProjectTemplate {
    pub logged_in: OptUser,
    pub project: Project,
    pub repos: Vec<Repo>,
}

#[derive(Template)]
#[template(path = "project/projects-list.html")]
pub struct ProjectsListTemplate {
    pub logged_in: OptUser,
    pub projects: Vec<Project>,
}

//# Group Templates

#[derive(Template)]
#[template(path = "group/group.html")]
pub struct GroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub meetings: Vec<Meeting>,
}

#[derive(Template)]
#[template(path = "group/groups-list.html")]
pub struct GroupsListTemplate {
    pub logged_in: OptUser,
    pub groups: Vec<Group>,
}

#[derive(Template)]
#[template(path = "group/new-group.html")]
pub struct NewGroupTemplate {
    pub logged_in: OptUser,
}

//# Calendar Templates

#[derive(Template)]
#[template(path = "calendar/calendar.html")]
pub struct CalendarTemplate {
    pub logged_in: OptUser,
    pub events: Vec<Event>,
}

#[derive(Template)]
#[template(path = "calendar/new-event.html")]
pub struct NewEventTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
}

//# Attendance Template

#[derive(Template)]
#[template(path = "attend.html")]
pub struct AttendTemplate {
    pub logged_in: OptUser,
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
