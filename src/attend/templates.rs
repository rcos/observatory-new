//! HTML templates for attendance

#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

/// Attendance page template
///
/// HTML File: `attend.html`
///
/// The page that shows the form for inputting an attendance code.
#[derive(Template)]
#[template(path = "attend.html")]
pub struct AttendTemplate {
    pub logged_in: OptUser,
}
