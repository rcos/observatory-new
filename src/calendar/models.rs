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
        format!("{} Event at: {}", self.title.clone(), self.start.clone())
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
        format!("/calendar/{}", self.id)
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

impl NewEvent {
    /// Verifies that the start and end times are valid
    /// by checking against a list of possible date/time formats
    pub fn fix_times(&mut self) -> Option<()> {
        // The array of possible valid strftime strings
        // and examples of what that format looks like.
        // https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html
        let fixed_times = vec![
            // 2018-05-03 10:22
            "%F %R", // 2018-05-03 10:22:11
            "%F %T", // 2018-05-03T10:22
            "%FT%R", // 2018-05-03T10:22:11
            "%FT%T", // 05/03/18 10:22
            "%D %R", // 05/03/18 10:22:11
            "%D %T", // 05/03/18 10:22 AM
            "%D %r",
        ]
        // Iterate thorugh
        .into_iter()
        // Turn into a tuple of start and end NaiveDateTimes
        // Assumes that both start and end have the same format
        .map(|s| {
            (
                NaiveDateTime::parse_from_str(&self.start, s),
                NaiveDateTime::parse_from_str(&self.end, s),
            )
        })
        // Find the first valid pair and stop
        .find_map(|e| match e {
            (Ok(s), Ok(e)) => Some((s, e)),
            _ => None,
        })?;

        // Set the start and end in self
        self.start = fixed_times.0.format("%F %R").to_string();
        self.end = fixed_times.1.format("%F %R").to_string();

        Some(())
    }
}
