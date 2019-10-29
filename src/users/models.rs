use crate::auth::crypto::UnsafeBinaryString;
use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub real_name: String,
    pub handle: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: UnsafeBinaryString,
    #[serde(skip)]
    pub salt: UnsafeBinaryString,
    pub bio: String,
    pub active: bool,
    pub joined_on: NaiveDateTime,
    pub tier: i32,
    pub mmost: String,
    pub former: bool,
    pub extrn: bool,
}

#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub real_name: String,
    pub handle: String,
    pub password_hash: UnsafeBinaryString,
    pub salt: UnsafeBinaryString,
    pub bio: String,
    pub email: String,
    pub tier: i32,
    pub active: bool,
    pub mmost: String,
    pub former: bool,
    pub extrn: bool,
}

use crate::models::Attendable;

#[derive(Debug, Default)]
pub struct GradeSummary {
    pub attendances: Vec<Box<dyn Attendable>>,
    pub needed_attendances: usize,
    pub commit_count: Option<usize>,
}
