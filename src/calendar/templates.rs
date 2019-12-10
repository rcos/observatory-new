//! HTML templates for the Calendar

use super::models::*;
#[allow(unused_imports)]
use crate::templates::filters;
use crate::templates::{FormError, OptUser};

use crate::models::User;

/// Calendar page template
///
/// HTML File: `calendar/calendar.html`
///
/// Displays either the nice FullCalendar view or a plain HTML list if
/// JS is disabled.
#[derive(Template)]
#[template(path = "calendar/calendar.html")]
pub struct CalendarTemplate {
    pub logged_in: OptUser,
    pub events: Vec<Event>,
}

/// Event page template
///
/// HTML File: `calendar/event.html`
///
/// Displays the information related to a single event
#[derive(Template)]
#[template(path = "calendar/event.html")]
pub struct EventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
    pub users: Vec<User>,
}

/// Template for creating a new Event
///
/// HTML File: `calendar/new-event.html`
///
/// Page for the form the create a new Event,
#[derive(Template)]
#[template(path = "calendar/new-event.html")]
pub struct NewEventTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
    pub error: Option<FormError>,
}

/// Template for editing an Event
///
/// HTML File: `calendar/edit-event.html`
///
/// Page for the form to edit an Event
#[derive(Template)]
#[template(path = "calendar/edit-event.html")]
pub struct EditEventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
    pub all_users: Vec<User>,
    pub error: Option<FormError>,
}
