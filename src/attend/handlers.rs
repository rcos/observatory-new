//! HTTP handlers for attendance codes

use diesel::insert_into;
use diesel::prelude::*;

use rocket::request::Form;
use rocket::response::Redirect;

use crate::guards::*;
use crate::ObservDbConn;

use super::code::*;
use super::models::*;
use super::templates::*;

/// GET handler for `/attend`
#[get("/attend")]
pub fn attend(l: UserGuard) -> AttendTemplate {
    AttendTemplate {
        logged_in: Some(l.0),
    }
}

/// An attendance code
///
/// Used to parse the incoming form in `attend_post`
#[derive(FromForm)]
pub struct AttendCode {
    code: String,
}

/// POST handler for `/attend`
///
/// Handles a POST request containing an attendance code that is being
/// submitted. If the code is valid it adds the attendance to the database
/// and redirects to `/`.
/// Otherwise redirects back to `/attend`.
#[post("/attend", data = "<code>")]
pub fn attend_post(conn: ObservDbConn, l: UserGuard, code: Form<AttendCode>) -> Redirect {
    use crate::schema::attendances::dsl::*;

    if let Some(m) = verify_code(&*conn, &code.code) {
        let (mid, eid) = if m.is_event() {
            (None, Some(m.id()))
        } else {
            (Some(m.id()), None)
        };
        let newattend = NewAttendance {
            user_id: l.0.id,
            is_event: m.is_event(),
            meeting_id: mid,
            event_id: eid,
        };
        insert_into(attendances)
            .values(&newattend)
            .execute(&*conn)
            .expect("Failed to insert attendance into database");
        Redirect::to("/")
    } else {
        Redirect::to("/attend")
    }
}
