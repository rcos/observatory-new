//# Database models
/// These should mirror the tables in schema.rs
/// **INCLUDING THE ORDER OF THE FIELDS**
use chrono::naive::NaiveDateTime;

use super::schema::*;

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub real_name: String,
    pub handle: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    #[serde(skip)]
    pub salt: String,
    pub active: bool,
    pub joined_on: NaiveDateTime,
    pub tier: i32,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub real_name: String,
    pub handle: String,
    pub password_hash: String,
    pub salt: String,
    pub email: String,
    pub tier: i32,
    pub active: bool,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Group)]
pub struct Meeting {
    pub id: i32,
    pub happened_at: NaiveDateTime,
    pub code: String,
    pub group_id: i32,
    pub hosted_by: i32,
}

impl Attendable for Meeting {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        format!("Meeting at: {}", self.happened_at)
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
    fn is_event(&self) -> bool {
        false
    }
    fn url(&self) -> String {
        format!("/h/{}", self.group_id)
    }
}

#[derive(Debug, Default, FromForm, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub code: String,
    pub group_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub active: bool,
    pub repos: String,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub repos: String,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Debug, Default, FromForm, Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Event {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub title: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
    pub color: Option<String>,
}

impl Attendable for Event {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        self.title.clone()
    }
    fn time(&self) -> NaiveDateTime {
        self.start
    }
    fn code(&self) -> String {
        self.code.clone()
    }
    fn owner_id(&self) -> i32 {
        self.hosted_by
    }
    fn is_event(&self) -> bool {
        true
    }
    fn url(&self) -> String {
        format!("/e/{}", self.id)
    }
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent {
    pub title: String,
    pub start: String,
    pub end: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
    pub color: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
pub struct Attendance {
    pub id: i32,
    pub user_id: i32,
    pub is_event: bool,
    pub meeting_id: Option<i32>,
    pub event_id: Option<i32>,
}

#[derive(Debug, Default, FromForm, Insertable)]
#[table_name = "attendances"]
pub struct NewAttendance {
    pub user_id: i32,
    pub is_event: bool,
    pub meeting_id: Option<i32>,
    pub event_id: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
#[table_name = "news"]
pub struct NewsStory {
    pub id: i32,
    pub happened_at: NaiveDateTime,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "news"]
pub struct NewNewsStory {
    pub happened_at: String,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
}

pub trait Attendable {
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn time(&self) -> NaiveDateTime;
    fn code(&self) -> String;
    fn owner_id(&self) -> i32;
    fn is_event(&self) -> bool;
    fn url(&self) -> String;
}

//# Relation Models

#[derive(Debug, PartialEq, Queryable, Identifiable)]
#[table_name = "relation_group_user"]
pub struct RelationGroupUser {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "relation_group_user"]
pub struct NewRelationGroupUser {
    pub group_id: i32,
    pub user_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Identifiable)]
#[table_name = "relation_project_user"]
pub struct RelationProjectUser {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "relation_project_user"]
pub struct NewRelationProjectUser {
    pub project_id: i32,
    pub user_id: i32,
}
