//! HTML templates used for groups

use super::models::*;
#[allow(unused_imports)]
use crate::templates::filters;
use crate::templates::{FormError, OptUser};

#[allow(unused_imports)]
use crate::models::Attendable;

use crate::models::User;

/// Attendance page template
///
/// HTML file: `group/group.html`
///
/// The page that shows the selected group's members and previous meetings
#[derive(Template)]
#[template(path = "group/group.html")]
pub struct GroupTemplate {
    /// Login information for the group
    pub logged_in: OptUser,
    /// Group that uses this template
    pub group: Group,
    /// Users this template is for
    pub users: Vec<User>,
    /// Meetings this group has had
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
    /// Login information for the group
    pub logged_in: OptUser,
    /// List of all users in group
    pub all_users: Vec<User>,
    /// The user gave invalid input so we tell them
    pub error: Option<FormError>,
}

/// Edit Group page template
///
/// HTML file: `group/edit-group.html`
///
/// The page that shows the same info as "New Group" for editing
#[derive(Template)]
#[template(path = "group/edit-group.html")]
pub struct EditGroupTemplate {
    /// Login information for the group
    pub logged_in: OptUser,
    /// Group that uses this template
    pub group: Group,
    /// List of all users in group
    pub all_users: Vec<User>,
    /// The user gave invalid input so we tell them
    pub error: Option<FormError>,
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
    /// LOgin information for the group
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

/// Add Meeting page template
///
/// HTML file: `group/meeting.html`
///
/// The page that shows the attedance code and attendees for a meeting
#[derive(Template)]
#[template(path = "group/meeting.html")]
pub struct MeetingTemplate {
    /// Login information for the group
    pub logged_in: OptUser,
    /// Group that contains this meeting
    pub group: Group,
    /// Users this template is for
    pub users: Vec<User>,
    /// Meeting that uses this template
    pub meeting: Meeting
}
