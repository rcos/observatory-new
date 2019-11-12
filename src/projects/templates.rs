use super::models::*;
#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

use crate::models::User;

/// Project page template
///
/// HTML File: `project/project.html`
///
/// Displays the project template or the plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "project/project.html")]
pub struct ProjectTemplate {
    pub logged_in: OptUser,
    pub project: Project,
    pub repos: Vec<String>,
    pub users: Vec<User>,
}

/// Project page template
///
/// HTML File: `project/new-project.html`
///
/// Displays the new-project template or the plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "project/new-project.html")]
pub struct NewProjectTemplate {
    pub logged_in: OptUser,
}

/// Project page template
///
/// HTML File: `project/edit-project.html`
///
/// Displays the edit-project template
#[derive(Template)]
#[template(path = "project/edit-project.html")]
pub struct EditProjectTemplate {
    pub logged_in: OptUser,
    pub project: Project,
    pub repos: Vec<String>,
    pub all_users: Vec<User>,
}

/// List of the Projects
///
/// HTML File: `project/project-list.html`
///
/// Displays the list of all projects
#[derive(Template)]
#[template(path = "project/projects-list.html")]
pub struct ProjectsListTemplate {
    pub logged_in: OptUser,
    pub projects: Vec<Project>,
    pub search_term: String,
    pub inactive: bool
}

/// Template shown when a student wants to join a project
///
/// HTML File: `project/join.html`
///
/// Shows a screen with some basic information for a student interested in joining the project
#[derive(Template)]
#[template(path = "project/join.html")]
pub struct JoinTemplate {
    pub logged_in: OptUser,
    pub project: Project,
}

/// The Adduser Template
///
/// HTML File: `project/project-list.html`
///
/// Displays the page when a student chooses to be added to a project
#[derive(Template)]
#[template(path = "project/add-user.html")]
pub struct AddUserTemplate {
    pub logged_in: OptUser,
    pub project: Project,
    pub all_users: Vec<User>,
}
