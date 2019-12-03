//!

use super::models::*;
use crate::models::{Group, Project};

#[allow(unused_imports)]
use crate::models::Attendable;
#[allow(unused_imports)]
use crate::templates::filters;

use crate::templates::{FormError, OptUser};

/// User page template
///
/// HTML File: `user/user.html`
///
/// Displays the user template or the plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "user/user.html")]
pub struct UserTemplate {
    pub logged_in: OptUser,
    pub user: User,
    pub projects: Vec<Project>,
    pub summary: GradeSummary,
    pub groups: Vec<Group>,
}

/// Edit-User page template
///
/// HTML File: `user/edit-user.html`
///
/// Displays the edit-user template or the plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "user/edit-user.html")]
pub struct EditUserTemplate {
    pub logged_in: OptUser,
    pub user: User,
    pub error: Option<FormError>,
}

/// UsersListTemplate page template
///
/// HTML File: `user/user-list.html`
///
/// Displays the user-list template or the plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "user/users-list.html")]
pub struct UsersListTemplate {
    pub logged_in: OptUser,
    pub users: Vec<User>,
    pub search_term: String,
    pub inactive: bool,
}
