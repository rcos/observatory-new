// These should mirror the tables in schema.rs

use rocket_contrib::databases::diesel::{Queryable, Insertable};
use super::schema::*;

#[derive(Queryable, Template)]
#[template(path = "user.html")]
pub struct User {
    pub id: i32,
    pub real_name: String,
    pub handle: String,
    pub email: String,
    pub password_hash: String,
    pub active: bool,
    pub joined_on: i32,
    pub tier: i32,
}

#[derive(Default, Insertable)]
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
    pub id: i32,
    pub datetime: i32,
    pub code: String,
    pub group_id: i32,
    pub hosted_by: i32,
}

#[derive(Default, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub datetime: i32,
    pub code: String,
    pub group_id: i32,
}

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub homepage: Option<String>,
    pub repo: String,
    pub owner_id: i32,
}

#[derive(Default, Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    name: String,
    homepage: Option<String>,
    repo: String,
    owner_id: i32,
}

#[derive(Queryable)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub room: Option<String>,
}

#[derive(Default, Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub room: Option<String>,
}
