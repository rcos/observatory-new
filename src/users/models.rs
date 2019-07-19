use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub real_name: String,
    pub handle: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    #[serde(skip)]
    pub salt: String,
    pub bio: String,
    pub active: bool,
    pub joined_on: NaiveDateTime,
    pub tier: i32,
    pub mmost: String,
}

#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub real_name: String,
    pub handle: String,
    pub password_hash: String,
    pub salt: String,
    pub bio: String,
    pub email: String,
    pub tier: i32,
    pub active: bool,
    pub mmost: String,
}

use crate::models::Attendable;

#[derive(Debug, Default)]
pub struct GradeSummary {
    pub attendances: Vec<Box<dyn Attendable>>,
    pub needed_attendances: usize,
    pub commit_count: Option<usize>,
}
