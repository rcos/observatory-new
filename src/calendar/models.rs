use chrono::naive::NaiveDateTime;

use crate::models::Attendable;
use crate::schema::*;

/// A calendar Event
#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Event {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub title: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
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

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent {
    pub title: String,
    pub start: String,
    pub end: String,
    pub description: Option<String>,
    pub hosted_by: i32,
    pub location: Option<String>,
    pub code: String,
    pub color: Option<String>,
}
