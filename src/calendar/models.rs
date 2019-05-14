//! Models for the calendar
//!
//! Calendar events are stored in the `calendar` table where each row
//! is an event.
use chrono::naive::NaiveDateTime;

use crate::models::Attendable;
use crate::schema::*;

/// A calendar Event
///
/// Represents an event on the calendar
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct Event {
    /// ID of the event
    pub id: i32,
    /// Time the event starts
    pub start: NaiveDateTime,
    /// Time the event ends
    pub end: NaiveDateTime,
    /// Title of the event
    pub title: String,
    /// Description of the event (optional)
    pub description: Option<String>,
    /// ID of the user hosting the event
    pub hosted_by: i32,
    /// Location the event (optional)
    pub location: Option<String>,
    /// Attendance code of the event
    pub code: String,
    /// Optional color to display the event on the calendar
    pub color: Option<String>,
}

// Implement the Attendable trait for an Event.
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

/// Used to create a new event in the database
#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent {
    /// Title of the event
    pub title: String,
    /// Time the event starts
    pub start: String,
    /// Time the event ends
    pub end: String,
    /// Description of the event (optional)
    pub description: Option<String>,
    /// ID of the user hosting the event
    pub hosted_by: i32,
    /// Location the event (optional)
    pub location: Option<String>,
    /// Attendance code of the event
    pub code: String,
    /// Optional color to display the event on the calendar
    pub color: Option<String>,
}
