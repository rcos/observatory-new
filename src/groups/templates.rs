use super::models::*;
#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

use crate::models::User;

#[derive(Template)]
#[template(path = "group/group.html")]
pub struct GroupTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub users: Vec<User>,
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
    pub all_users: Vec<User>,
}

#[derive(Template)]
#[template(path = "group/groups-list.html")]
pub struct GroupsListTemplate {
    pub logged_in: OptUser,
    pub groups: Vec<Group>,
}

#[derive(Template)]
#[template(path = "group/add-user.html")]
pub struct AddUserTemplate {
    pub logged_in: OptUser,
    pub group: Group,
    pub all_users: Vec<User>,
}
