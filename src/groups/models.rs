//! Models for groups
//!
//! Groups are stored in the `groups` table where each row is a group member
//! or a meeting

use chrono::NaiveDateTime;

use crate::models::Attendable;
use crate::models::User;
use crate::schema::*;

/// Models a group in the database
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct Group {
    /// ID of the group
    pub id: i32,
    /// Name of the group
    pub name: String,
    /// ID of the group mentor
    pub owner_id: i32,
    /// Meeting location
    pub location: Option<String>,
}

/// Used to create a new group in the database
#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "groups"]
pub struct NewGroup {
    /// Name of the group
    pub name: String,
    /// ID of the group mentor
    pub owner_id: i32,
    /// Meeting location
    pub location: Option<String>,
}

/// Models a meeting in the database
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Group)]
pub struct Meeting {
    /// ID of the meeting
    pub id: i32,
    /// Time of the meeting
    pub happened_at: NaiveDateTime,
    /// Attendance code
    pub code: String,
    /// ID of the group
    pub group_id: i32,
    /// ID of the mentor who hosted the meeting/event
    pub hosted_by: i32,
}

impl Attendable for Meeting {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        format!("Meeting on: {}", self.happened_at.format("%b. %-d, at %l:%M %p").to_string())
    }
    fn time(&self) -> NaiveDateTime {
        self.happened_at
    }
    fn code(&self) -> String {
        self.code.clone()
    }
    fn owner_id(&self) -> i32 {
        self.hosted_by
    }
    fn group_id(&self) -> Option<i32> {
        if !self.is_event() {
            Some(self.group_id)
        } else {
            None
        }
    }
    fn is_event(&self) -> bool {
        false
    }
    fn url(&self) -> String {
        format!("/groups/{}/meetings/{}", self.group_id, self.id)
    }
}

/// Used to create a new meeting in the database
#[derive(Debug, Default, Clone, FromForm, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    /// Attendance code
    pub code: String,
    /// ID of the group
    pub group_id: i32,
}

/// Models the relationship of a user between different groups
#[derive(Debug, PartialEq, Clone, Queryable, Associations, Identifiable)]
#[belongs_to(Group)]
#[belongs_to(User)]
#[table_name = "relation_group_user"]
pub struct RelationGroupUser {
    /// ID of the user's original group
    pub id: i32,
    /// ID of another group that the user belongs to
    pub group_id: i32,
    /// ID of the user
    pub user_id: i32,
}

/// Used to create a new group relationship for a user
#[derive(Debug, Default, Clone, Insertable)]
#[table_name = "relation_group_user"]
pub struct NewRelationGroupUser {
    /// ID of the group the user is joining   
    pub group_id: i32,
    /// ID of the user
    pub user_id: i32,
}
