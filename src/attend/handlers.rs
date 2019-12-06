//! HTTP handlers for attendance codes

use diesel::insert_into;
use diesel::prelude::*;

use rocket::request::Form;
use rocket::response::Redirect;

use crate::guards::*;
use crate::models::RelationGroupUser;
use crate::templates::FormError;
use crate::ObservDbConn;

use super::code::*;
use super::models::*;
use super::templates::*;

/// GET handler for `/attend`
#[get("/attend?<e>")]
pub fn attend(l: UserGuard, e: Option<FormError>) -> AttendTemplate {
    AttendTemplate {
        logged_in: Some(l.0),
        error: e,
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
    if let Some(m) = verify_code(&*conn, &code.code) {
        let (mid, eid, gid) = if m.is_event() {
            (None, Some(m.id()), None)
        } else {
            (Some(m.id()), None, m.group_id())
        };

        let user_in_group = if !m.is_event() {
            use crate::schema::relation_group_user::dsl::*;
            relation_group_user
                .filter(group_id.eq(gid.unwrap()).and(user_id.eq(l.0.id)))
                .first::<RelationGroupUser>(&*conn)
                .optional()
                .expect("Failed to get relations from database")
                .is_some()
        } else {
            false
        };

        use crate::schema::attendances::dsl::*;
        let user_attended = if m.is_event() {
            attendances
                .filter(event_id.eq(eid).and(user_id.eq(l.0.id)))
                .first::<Attendance>(&*conn)
                .optional()
                .expect("Failed to get attendances from database")
                .is_some()
        } else {
            attendances
                .filter(meeting_id.eq(mid).and(user_id.eq(l.0.id)))
                .first::<Attendance>(&*conn)
                .optional()
                .expect("Failed to get attendances from database")
                .is_some()
        };

        if !user_attended && (m.is_event() || (!m.is_event() && user_in_group)) {
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
            Redirect::to("/dashboard")
        } else {
            Redirect::to(format!("/attend?e={}", FormError::UsedCode))
        }
    } else {
        Redirect::to(format!("/attend?e={}", FormError::InvalidCode))
    }
}
