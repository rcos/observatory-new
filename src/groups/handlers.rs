use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::attend::code::attendance_code;
use crate::guards::*;
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;

#[get("/groups/<gid>")]
pub fn group(conn: ObservDbConn, l: UserGuard, gid: i32) -> Option<GroupTemplate> {
    use crate::schema::groups::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .optional()
        .expect("Failed to get groups from database")?;

    let m: Vec<Meeting> = Meeting::belonging_to(&g)
        .load(&*conn)
        .expect("Failed to get project's repos from database");

    Some(GroupTemplate {
        logged_in: Some(l.0),
        group: g,
        meetings: m,
    })
}

#[get("/groups")]
pub fn groups(conn: ObservDbConn, l: MentorGuard) -> GroupsListTemplate {
    use crate::schema::groups::dsl::*;
    GroupsListTemplate {
        logged_in: Some(l.0),
        groups: groups
            .load(&*conn)
            .expect("Failed to get groups from database"),
    }
}

#[get("/groups/new")]
pub fn newgroup(conn: ObservDbConn, l: AdminGuard) -> NewGroupTemplate {
    use crate::schema::users::dsl::*;
    NewGroupTemplate {
        logged_in: Some(l.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from database"),
    }
}

#[post("/groups/new", data = "<newgroup>")]
pub fn newgroup_post(conn: ObservDbConn, _l: AdminGuard, newgroup: Form<NewGroup>) -> Redirect {
    use crate::schema::groups::dsl::*;

    insert_into(groups)
        .values(&newgroup.into_inner())
        .execute(&*conn)
        .expect("Failed to insert group into database");
    Redirect::to("/groups")
}

#[post("/groups/<gid>", data = "<newmeeting>")]
pub fn newmeeting_post(
    conn: ObservDbConn,
    _l: MentorGuard,
    gid: i32,
    newmeeting: Form<NewMeeting>,
) -> Redirect {
    use crate::schema::meetings::dsl::*;

    let mut newmeeting = newmeeting.into_inner();
    newmeeting.group_id = gid;
    newmeeting.code = attendance_code(&*conn);

    insert_into(meetings)
        .values(&newmeeting)
        .execute(&*conn)
        .expect("Failed to insert meeting into database");

    Redirect::to(format!("/groups/{}", newmeeting.group_id))
}

#[get("/groups/<gid>/edit")]
pub fn editgroup(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
) -> Result<EditGroupTemplate, Status> {
    use crate::schema::groups::dsl::*;
    use crate::schema::users::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");

    if l.0.tier > 1 || g.owner_id == l.0.id {
        Ok(EditGroupTemplate {
            logged_in: Some(l.0),
            group: g,
            all_users: users
                .load(&*conn)
                .expect("Failed to get users from database"),
        })
    } else {
        Err(Status::Unauthorized)
    }
}

#[put("/groups/<gid>", data = "<editgroup>")]
pub fn editgroup_put(
    conn: ObservDbConn,
    l: MentorGuard,
    editgroup: Form<NewGroup>,
    gid: i32,
) -> Result<Redirect, Status> {
    use crate::schema::groups::dsl::*;

    let mut editgroup = editgroup.into_inner();

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");

    if l.0.tier > 1 || g.owner_id == l.0.id {
        if !(l.0.tier > 1) {
            editgroup.owner_id = l.0.id;
        }
        update(groups.find(gid))
            .set(&editgroup)
            .execute(&*conn)
            .expect("Failed to update group in the database");
        Ok(Redirect::to(format!("/groups/{}", gid)))
    } else {
        Err(Status::Unauthorized)
    }
}

#[delete("/groups/<gid>")]
pub fn group_delete(conn: ObservDbConn, _l: AdminGuard, gid: i32) -> Redirect {
    use crate::schema::groups::dsl::*;
    delete(groups.find(gid))
        .execute(&*conn)
        .expect("Failed to delete group from database");
    Redirect::to("/groups")
}

use crate::users::models::User;
fn group_users(conn: &SqliteConnection, group: Group) -> Vec<User> {
    unimplemented!()
}