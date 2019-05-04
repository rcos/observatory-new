use crate::models::User;
use crate::schema::*;

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
pub struct Attendance {
    pub id: i32,
    pub is_event: bool,
    pub user_id: i32,
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
