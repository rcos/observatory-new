//! Models for user attendance.
//!
//! Attendance is stored in the `attendance` table where each
//! row is a time someone attended something.

use crate::models::Meeting;
use crate::models::User;
use crate::schema::*;

/// Models an attendance in the database
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Meeting)]
#[belongs_to(User)]
pub struct Attendance {
    /// ID of the attendance
    pub id: i32,
    /// Was it an event that they attended?
    pub is_event: bool,
    /// The user this attendance belongs to
    pub user_id: i32,
    /// If `is_event` is false this will be the meeting they attended
    pub meeting_id: Option<i32>,
    /// If `is_event` is true this will be the event they attended
    pub event_id: Option<i32>,
}

/// Used to create a new attendance in the database
#[derive(Debug, Default, Clone, FromForm, Insertable)]
#[table_name = "attendances"]
pub struct NewAttendance {
    /// The user this attendance belongs to
    pub user_id: i32,
    /// Was it an event that they attended?
    pub is_event: bool,
    /// If `is_event` is false this will be the meeting they attended
    pub meeting_id: Option<i32>,
    /// If `is_event` is true this will be the event they attended
    pub event_id: Option<i32>,
}
