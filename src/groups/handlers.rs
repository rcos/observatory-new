//! HTTP handlers for the calendar

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::attend::code::attendance_code;
use crate::guards::*;
use crate::templates::{is_reserved, FormError};
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;

/// GET handler for `/groups/<gid>`
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
        users: group_users(&*conn, &g),
        group: g,
        meetings: m,
    })
}

/// GET handler for `/groups`
///
/// Returns a list of groups
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

/// GET handler for `/groups.json`
///
/// JSON endpoint that returns the list of groups as a single JSON array
#[get("/groups.json")]
pub fn groups_json(conn: ObservDbConn, _l: MentorGuard) -> Json<Vec<Group>> {
    use crate::schema::groups::dsl::*;
    Json(
        groups
            .load(&*conn)
            .expect("Failed to get groups from database"),
    )
}

/// GET handler for `/groups/new`
///
/// Creates a new group list and populates it with users
#[get("/groups/new?<e>")]
pub fn group_new(conn: ObservDbConn, l: AdminGuard, e: Option<FormError>) -> NewGroupTemplate {
    use crate::schema::users::dsl::*;
    NewGroupTemplate {
        logged_in: Some(l.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from database"),
        error: e,
    }
}

/// POST handler for `/groups/new`
///
/// Creates a new group list. For use with `newgroup`.
///
/// Restricted to Admins
#[post("/groups/new", data = "<newgroup>")]
pub fn group_new_post(conn: ObservDbConn, _l: AdminGuard, newgroup: Form<NewGroup>) -> Redirect {
    let newgroup = newgroup.into_inner();

    if let Err(e) = is_reserved(&newgroup.name) {
        return Redirect::to(format!("/groups/new?e={}", e));
    }

    use crate::schema::groups::dsl::*;
    insert_into(groups)
        .values(&newgroup)
        .execute(&*conn)
        .expect("Failed to insert group into database");

    use crate::schema::groups::dsl::id;
    let gid = groups
        .filter(name.eq(&*newgroup.name).and(owner_id.eq(newgroup.owner_id)))
        .select(id)
        .first(&*conn)
        .expect("Failed to get group from database");

    use crate::schema::relation_group_user::dsl::*;
    insert_into(relation_group_user)
        .values(&NewRelationGroupUser {
            group_id: gid,
            user_id: newgroup.owner_id,
        })
        .execute(&*conn)
        .expect("Failed to insert relation into database");

    audit_logger!(
        "User {} [{}] has created Group {} \'{}\'",
        _l.0.id,
        _l.0.email,
        gid,
        newgroup.name
    );

    Redirect::to("/groups")
}

/// GET handler for `/groups/<gid>/meetings`
#[get("/groups/<gid>/meetings")]
pub fn meetings(gid: i32) -> Redirect {
    Redirect::to(format!("/groups/{}", gid))
}

/// GET handler for `/groups/<gid>/meetings.json`
#[get("/groups/<gid>/meetings.json")]
pub fn meetings_json(conn: ObservDbConn, _l: MentorGuard, gid: i32) -> Json<Vec<Meeting>> {
    use crate::schema::meetings::dsl::*;
    Json(
        meetings
            .filter(group_id.eq(gid))
            .load(&*conn)
            .expect("Failed to get meetings from database"),
    )
}

/// POST handler for `/groups/<gid>/meetings/new`
///
/// Records a new meeting
#[post("/groups/<gid>/meetings/new", data = "<newmeeting>")]
pub fn meeting_new_post(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
    newmeeting: Form<NewMeeting>,
) -> Redirect {
    use crate::schema::groups::dsl::*;

    audit_logger!(
        "User {} [{}] has generated an attendance code for Group {}",
        l.0.id,
        l.0.email,
        gid
    );

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");

    if l.0.tier > 1
        || l.0.id == g.owner_id
        || (l.0.id > 0 && group_users(&*conn, &g).contains(&l.0) && g.id > 0)
    {
        use crate::schema::meetings::dsl::*;
        let mut newmeeting = newmeeting.into_inner();
        newmeeting.group_id = gid;
        newmeeting.code = attendance_code(&*conn);

        insert_into(meetings)
            .values(&newmeeting)
            .execute(&*conn)
            .expect("Failed to insert meeting into database");
    }
    Redirect::to(format!("/groups/{}", gid))
}

/// GET handler for `/groups/<gid>/members/add`
///
/// Returns a list of users for a given group in order to add a member
#[get("/groups/<gid>/members/add")]
pub fn group_user_add(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
) -> Result<AddUserTemplate, Status> {
    use crate::schema::groups::dsl::*;
    use crate::schema::users::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");
    let all_users: Vec<User> = users
        .load(&*conn)
        .expect("Failed to get users from database");
    let gu = group_users(&*conn, &g);

    if l.0.tier > 1 || g.owner_id == l.0.id {
        Ok(AddUserTemplate {
            logged_in: Some(l.0),
            group: g,
            all_users: all_users
                .iter()
                .filter(|&e| !gu.contains(e))
                .cloned()
                .collect(),
        })
    } else {
        Err(Status::Unauthorized)
    }
}

///
#[derive(FromForm)]
pub struct AddUserForm {
    uid: Option<i32>,
}

/// POST handler `/groups/<gid>/members/add`
///
/// Adds a user to a group  
#[post("/groups/<gid>/members/add", data = "<form>")]
pub fn group_user_add_post(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
    form: Form<AddUserForm>,
) -> Result<Redirect, Status> {
    use crate::schema::groups::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");

    if l.0.tier > 1 || g.owner_id == l.0.id {
        use crate::schema::relation_group_user::dsl::*;

        if let Some(uid) = form.into_inner().uid {
            insert_into(relation_group_user)
                .values(&NewRelationGroupUser {
                    group_id: g.id,
                    user_id: uid,
                })
                .execute(&*conn)
                .expect("Failed to insert new relation into database");

            audit_logger!(
                "User {} [{}] has added User {} to Group {}",
                l.0.id,
                l.0.email,
                uid,
                g.id
            );

            Ok(Redirect::to(format!("/groups/{}", gid)))
        } else {
            Ok(Redirect::to("/"))
        }
    } else {
        Err(Status::Unauthorized)
    }
}

/// DELETE handler for `/groups/<gid>/members/<uid>`
///
/// Deletes a member from a group
#[delete("/groups/<gid>/members/<uid>")]
pub fn group_user_delete(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
    uid: i32,
) -> Result<Redirect, Status> {
    use crate::schema::groups::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .expect("Failed to get group from database");

    if l.0.tier > 1 || g.owner_id == l.0.id {
        use crate::schema::relation_group_user::dsl::*;
        delete(relation_group_user.filter(group_id.eq(g.id).and(user_id.eq(uid))))
            .execute(&*conn)
            .expect("Failed to removed user from group in database");
        Ok(Redirect::to(format!("/groups/{}", gid)))
    } else {
        Err(Status::Unauthorized)
    }
}

/// GET handler for `/groups/<gid>/edit`
///
/// Returns a list of group members for the mentor
#[get("/groups/<gid>/edit?<e>")]
pub fn group_edit(
    conn: ObservDbConn,
    l: MentorGuard,
    gid: i32,
    e: Option<FormError>,
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
            error: e,
        })
    } else {
        Err(Status::Unauthorized)
    }
}

/// PUT handler for `/groups/<gid>`
///
/// Updates the group owner
#[put("/groups/<gid>", data = "<editgroup>")]
pub fn group_edit_put(
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
        if let Err(e) = is_reserved(&editgroup.name) {
            return Ok(Redirect::to(format!("/groups/{}/edit?e={}", gid, e)));
        }

        if l.0.tier <= 1 {
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

/// DELETE handler for `/groups/<gid>`
///
/// Deletes a group from the database
#[delete("/groups/<gid>")]
pub fn group_delete(conn: ObservDbConn, _l: AdminGuard, gid: i32) -> Redirect {
    use crate::schema::relation_group_user::dsl::*;
    delete(relation_group_user.filter(group_id.eq(gid)))
        .execute(&*conn)
        .expect("Failed to delete relations from database");
    use crate::schema::groups::dsl::*;
    delete(groups.find(gid))
        .execute(&*conn)
        .expect("Failed to delete group from database");
    Redirect::to("/groups")
}

/// Returns a list of users in a given group
use crate::models::User;
fn group_users(conn: &SqliteConnection, group: &Group) -> Vec<User> {
    RelationGroupUser::belonging_to(group)
        .load::<RelationGroupUser>(conn)
        .expect("Failed to get relations from database")
        .iter()
        .map(|r| {
            use crate::schema::users::dsl::*;
            users
                .find(r.user_id)
                .first(conn)
                .expect("Failed to get user from database")
        })
        .collect()
}
