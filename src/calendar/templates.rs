use super::models::*;
#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

use crate::users::User;

#[derive(Template)]
#[template(path = "calendar/calendar.html")]
pub struct CalendarTemplate {
    pub logged_in: OptUser,
    pub events: Vec<Event>,
}

#[derive(Template)]
#[template(path = "calendar/event.html")]
pub struct EventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
}

#[derive(Template)]
#[template(path = "calendar/new-event.html")]
pub struct NewEventTemplate {
    pub logged_in: OptUser,
    pub all_users: Vec<User>,
}

#[derive(Template)]
#[template(path = "calendar/edit-event.html")]
pub struct EditEventTemplate {
    pub logged_in: OptUser,
    pub event: Event,
    pub all_users: Vec<User>,
}
