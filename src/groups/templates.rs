//! HTML templates used for groups

use super::models::*;
#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

use crate::models::User;

/// Attendance page template
///
/// HTML file: `group/group.html`
///
/// The page that shows the selected group's members and previous meetings
#[derive(Template)]
#[template(path = "group/group.html")]
pub struct GroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub users: Vec<User>,
    pub meetings: Vec<Meeting>,
}

/// New Group page template
///
/// HTML file: `group/new-group.html`
///
/// The page that shows the form for entering a new group,
/// meeting location, and group leader
#[derive(Template)]
#[template(path = "group/new-group.html")]
pub struct NewGroupTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
}

/// Edit Group page template
///
/// HTML file: `group/edit-group.html`
///
/// The page that shows the same info as "New Group" for editing
#[derive(Template)]
#[template(path = "group/edit-group.html")]
pub struct EditGroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub all_users: Vec<User>,
}

/// Groups List page template
///
/// HTML file: `group/groups-list.html`
///
/// The page that shows a list of all groups
#[derive(Template)]
#[template(path = "group/groups-list.html")]
pub struct GroupsListTemplate {
    pub logged_in: OptUser,
    pub groups: Vec<Group>,
}

/// Add User page template
///
/// HTML file: `group/add-user.html`
///
/// The page that shows the form for adding a registered user to a group
#[derive(Template)]
#[template(path = "group/add-user.html")]
pub struct AddUserTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub all_users: Vec<User>,
}
