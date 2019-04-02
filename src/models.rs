// These should mirror the tables in schema.rs

use super::schema::*;

#[derive(Queryable, Serialize, Template)]
#[template(path = "user.html")]
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
    pub joined_on: String,
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

#[derive(Queryable)]
pub struct Meeting {
    pub id: i32,
    pub happened_on: String,
    pub code: String,
    pub group_id: i32,
    pub hosted_by: i32,
}

#[derive(Default, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub code: String,
    pub group_id: i32,
}

#[derive(Queryable, Serialize, Template)]
#[template(path = "project.html")]
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

#[derive(Queryable, Serialize, Template)]
#[template(path = "group.html")]
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
