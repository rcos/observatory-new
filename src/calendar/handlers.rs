//! HTTP handlers for the calendar

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::json::Json;

use crate::attend::code::attendance_code;
use crate::guards::*;
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;

/// GET handler for `/calendar`
#[get("/calendar")]
pub fn calendar(conn: ObservDbConn, l: MaybeLoggedIn) -> CalendarTemplate {
    use crate::schema::events::dsl::*;

    CalendarTemplate {
        logged_in: l.user(),
        events: events
            .order(start.asc())
            .load(&conn.0)
            .expect("Failed to get events"),
    }
}

/// GET handler for `/calendar.json`
///
/// JSON endpoint that returns the calendar events as a single JSON array.
#[get("/calendar.json")]
pub fn calendar_json(conn: ObservDbConn) -> Json<Vec<Event>> {
    use crate::schema::events::dsl::*;

    Json(
        events
            .order(start.asc())
            .load(&*conn)
            .expect("Failed to get events"),
    )
}

/// GET handler for `/calendar/<eid>`
#[get("/calendar/<eid>")]
pub fn event(conn: ObservDbConn, l: MaybeLoggedIn, eid: i32) -> Option<EventTemplate> {
    use crate::schema::events::dsl::*;

    Some(EventTemplate {
        logged_in: l.user(),
        event: events
            .find(eid)
            .first(&*conn)
            .optional()
            .expect("Failed to get event")?,
    })
}

/// GET handler for `/calendar/<eid>/edit`
#[get("/calendar/<eid>/edit")]
pub fn editevent(conn: ObservDbConn, l: AdminGuard, eid: i32) -> Option<EditEventTemplate> {
    use crate::schema::events::dsl::*;
    use crate::schema::users::dsl::*;
    Some(EditEventTemplate {
        logged_in: Some(l.0),
        event: events
            .find(eid)
            .first(&*conn)
            .optional()
            .expect("Failed to get event from database")?,
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from database"),
    })
}

/// PUT handler for `/calendar/<eid>`
#[put("/calendar/<eid>", data = "<editevent>")]
pub fn editevent_put(
    conn: ObservDbConn,
    l: UserGuard,
    eid: i32,
    editevent: Form<NewEvent>,
) -> Result<Redirect, Status> {
    let l = l.0;

    use crate::schema::events::dsl::*;
    let mut editevent = editevent.into_inner();
    let (atcode, host_id): (String, i32) = events
        .find(eid)
        .select((code, hosted_by))
        .first(&*conn)
        .expect("Failed to get event code");
    editevent.code = atcode;

    if l.tier > 1 || l.id == host_id {
        update(events.find(eid))
            .set(&editevent)
            .execute(&*conn)
            .expect("Failed to update event in database");

        Ok(Redirect::to("/calendar"))
    } else {
        Err(Status::Unauthorized)
    }
}

/// DELETE handler for `/calendar/<eid>
#[delete("/calendar/<eid>")]
pub fn event_delete(conn: ObservDbConn, _l: AdminGuard, eid: i32) -> Redirect {
    use crate::schema::events::dsl::*;
    delete(events.find(eid))
        .execute(&*conn)
        .expect("Failed to delete event from database");
    Redirect::to("/calendar")
}

/// GET handler for `/calendar/new`
#[get("/calendar/new")]
pub fn newevent(conn: ObservDbConn, admin: AdminGuard) -> NewEventTemplate {
    use crate::schema::users::dsl::*;
    NewEventTemplate {
        logged_in: Some(admin.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from database"),
    }
}

/// POST handler for `/calendar/new`
#[post("/calendar/new", data = "<newevent>")]
pub fn newevent_post(conn: ObservDbConn, _admin: AdminGuard, newevent: Form<NewEvent>) -> Redirect {
    use crate::schema::events::dsl::*;

    let mut newevent = newevent.into_inner();
    newevent.code = attendance_code(&*conn);

    insert_into(events)
        .values(&newevent)
        .execute(&*conn)
        .expect("Failed to add user to database");

    Redirect::to("/calendar")
}
