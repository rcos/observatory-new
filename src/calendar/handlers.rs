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
///
/// Returns the main calendar page which either shows the FullCalendar view
/// or a plain HTML list if JS is off.
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
///
/// A single calendar event's page with information on the event.
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
///
/// The page to edit a calendar event.
///
/// Restricted to Admins and the event owner.
#[get("/calendar/<eid>/edit")]
pub fn event_edit(conn: ObservDbConn, l: UserGuard, eid: i32) -> Result<EditEventTemplate, Status> {
    let l = l.0;

    use crate::schema::events::dsl::*;
    use crate::schema::users::dsl::*;

    let host_id: i32 = events
        .find(eid)
        .select(hosted_by)
        .first(&*conn)
        .expect("Failed to get event code");

    if l.tier > 1 || l.id == host_id {
        Ok(EditEventTemplate {
            logged_in: Some(l),
            event: if let Some(e) = events
                .find(eid)
                .first(&*conn)
                .optional()
                .expect("Failed to get event from database")
            {
                e
            } else {
                // Return early
                return Err(Status::NotFound);
            },
            all_users: users
                .load(&*conn)
                .expect("Failed to get users from database"),
        })
    } else {
        Err(Status::Unauthorized)
    }
}

/// PUT handler for `/calendar/<eid>`
///
/// Changes the calendar event. For use with `editevent`.
///
/// Restricted to Admins and the event owner.
#[put("/calendar/<eid>", data = "<editevent>")]
pub fn event_edit_put(
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
///
/// Deletes an event from the calendar and database.
///
/// Restricted to Admins.
#[delete("/calendar/<eid>")]
pub fn event_delete(conn: ObservDbConn, _l: AdminGuard, eid: i32) -> Redirect {
    use crate::schema::events::dsl::*;
    delete(events.find(eid))
        .execute(&*conn)
        .expect("Failed to delete event from database");
    Redirect::to("/calendar")
}

/// GET handler for `/calendar/new`
///
/// Template to create a new calendar event.
///
/// Restricted to Admins.
#[get("/calendar/new")]
pub fn event_new(conn: ObservDbConn, admin: AdminGuard) -> NewEventTemplate {
    use crate::schema::users::dsl::*;
    NewEventTemplate {
        logged_in: Some(admin.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from database"),
    }
}

/// POST handler for `/calendar/new`
///
/// Creates the new calendar event. For use with `newevent`.
///
/// Restricted to Admins.
#[post("/calendar/new", data = "<newevent>")]
pub fn event_new_post(
    conn: ObservDbConn,
    _admin: AdminGuard,
    newevent: Form<NewEvent>,
) -> Redirect {
    use crate::schema::events::dsl::*;

    let mut newevent = newevent.into_inner();
    newevent.code = attendance_code(&*conn);

    insert_into(events)
        .values(&newevent)
        .execute(&*conn)
        .expect("Failed to add user to database");

    Redirect::to("/calendar")
}
