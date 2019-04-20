use crate::models::*;

type OptUser = Option<User>;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: OptUser,
    pub version: &'static str,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub logged_in: OptUser,
}

//# Sign Up and Log In templates

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

//# User Templates

#[derive(Template)]
#[template(path = "user/user.html")]
pub struct UserTemplate {
    pub logged_in: OptUser,
    pub user: User,
}

#[derive(Template)]
#[template(path = "user/edit-user.html")]
pub struct EditUserTemplate {
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
    pub repos: Vec<String>,
}

#[derive(Template)]
#[template(path = "project/new-project.html")]
pub struct NewProjectTemplate {
    pub logged_in: OptUser,
}

#[derive(Template)]
#[template(path = "project/edit-project.html")]
pub struct EditProjectTemplate {
    pub logged_in: OptUser,
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
#[template(path = "group/new-group.html")]
pub struct NewGroupTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
}

#[derive(Template)]
#[template(path = "group/edit-group.html")]
pub struct EditGroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
}

#[derive(Template)]
#[template(path = "group/groups-list.html")]
pub struct GroupsListTemplate {
    pub logged_in: OptUser,
    pub groups: Vec<Group>,
}

//# Calendar Templates

#[derive(Template)]
#[template(path = "calendar/calendar.html")]
pub struct CalendarTemplate {
    pub logged_in: OptUser,
    pub events: Vec<Event>,
}

#[derive(Template)]
#[template(path = "calendar/event.html")]
pub struct EventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
}

#[derive(Template)]
#[template(path = "calendar/new-event.html")]
pub struct NewEventTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
}

#[derive(Template)]
#[template(path = "calendar/edit-event.html")]
pub struct EditEventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
    pub all_users: Vec<User>,
}

//# Attendance Template

#[derive(Template)]
#[template(path = "attend.html")]
pub struct AttendTemplate {
    pub logged_in: OptUser,
}

//# News Templates
#[derive(Template)]
#[template(path = "news/news.html")]
pub struct NewsTemplate {
    pub logged_in: OptUser,
    pub stories: Vec<NewsStory>,
}

#[derive(Template)]
#[template(path = "news/newsstory.html")]
pub struct NewsStoryTemplate {
    pub logged_in: OptUser,
    pub story: NewsStory,
}

#[derive(Template)]
#[template(path = "news/new-newsstory.html")]
pub struct NewNewsStoryTemplate {
    pub logged_in: OptUser,
}

#[derive(Template)]
#[template(path = "news/edit-newsstory.html")]
pub struct EditNewsStoryTemplate {
    pub logged_in: OptUser,
    pub story: NewsStory,
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
