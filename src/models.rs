// These should mirror the tables in scheme.rs

use super::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: u32,
    pub real_name: String,
    pub handle: String,
    pub email: String,
    pub active: bool,
    pub joined_on: u32,
    pub tier: u32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub real_name: String,
    pub handle: String,
    pub password_hash: String,
    pub email: String,
    pub active: bool,
}

#[derive(Queryable)]
pub struct Meeting {
    pub id: u32,
    pub datetime: u32,
    pub code: String,
    pub group_id: u32,
}

#[derive(Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub datetime: i32,
    pub code: String,
    pub group_id: i32,
}

#[derive(Queryable)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub homepage: Option<String>,
    pub repo: String,
    pub owner_id: u32,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    name: String,
    homepage: Option<String>,
    repo: String,
    owner_id: i32,
}

#[derive(Queryable)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub owner_id: u32,
    pub room: Option<String>,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub room: Option<String>,
}
