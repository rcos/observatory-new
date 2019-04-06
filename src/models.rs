// These should mirror the tables in schema.rs

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

#[derive(Default, FromForm, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub real_name: String,
    pub handle: String,
    pub password_hash: String,
    pub salt: String,
    pub email: String,
}

#[derive(Default, FromForm)]
pub struct LogInForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
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
        format!("Meeting at: {}", self.hosted_by)
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
}

#[derive(Default, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub code: String,
    pub group_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub active: bool,
}

#[derive(Default, Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Default, Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Project)]
pub struct Repo {
    pub id: i32,
    pub project_id: i32,
    pub url: String,
}

#[derive(Default, Insertable)]
#[table_name = "repos"]
pub struct NewRepo {
    pub project_id: i32,
    pub url: String,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Event {
    pub id: i32,
    pub happening_at: NaiveDateTime,
    pub title: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
}

impl Attendable for Event {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        self.title.clone()
    }
    fn time(&self) -> NaiveDateTime {
        self.happening_at
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
}

#[derive(Default, FromForm, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub happening_at: String,
    pub title: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Attendance {
    pub id: i32,
    pub user_id: i32,
    pub is_event: bool,
    pub meeting_id: Option<i32>,
    pub event_id: Option<i32>,
}

#[derive(Default, FromForm, Insertable)]
#[table_name = "attendances"]
pub struct NewAttendance {
    pub user_id: i32,
    pub is_event: bool,
    pub meeting_id: Option<i32>,
    pub event_id: Option<i32>,
}

pub trait Attendable {
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn time(&self) -> NaiveDateTime;
    fn code(&self) -> String;
    fn owner_id(&self) -> i32;
    fn is_event(&self) -> bool;
}
