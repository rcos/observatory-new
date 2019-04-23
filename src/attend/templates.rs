#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

#[derive(Template)]
#[template(path = "attend.html")]
pub struct AttendTemplate {
    pub logged_in: OptUser,
}
