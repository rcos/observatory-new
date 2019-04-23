use crate::templates::OptUser;

#[derive(Template)]
#[template(path = "attend.html")]
pub struct AttendTemplate {
    pub logged_in: OptUser,
}
