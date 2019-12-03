//! The handler for the user page this code handles the function of searching for users, creating users, deleting users
//! Checking users relation to a project, number of commits made, return a list of users, and the user's grade summary 

use diesel::prelude::*;
use diesel::{delete, update};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::json::Json;

use crate::auth::crypto::*;
use crate::guards::*;
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;

/// GET handler for '/users/<h>'
/// Gets an Indivual user by their ID and returns it to the template

#[get("/users/<h>")]
pub fn user(conn: ObservDbConn, l: MaybeLoggedIn, h: i32) -> Option<UserTemplate> {
    use crate::schema::users::dsl::*;

    let u = users
        .find(h)
        .first(&*conn)
        .optional()
        .expect("Failed to get user from database")?;

    Some(UserTemplate {
        logged_in: l.user(),
        projects: user_projects(&*conn, &u),
        groups: user_groups(&*conn, &u),
        summary: grade_summary(&*conn, &u),
        user: u,
    })
}

/// GET handler for '/users/<h>'
/// Gets an indivual user by there Github handle and redirects them to their user ID

#[get("/users/<h>", rank = 2)]
pub fn user_by_handle(conn: ObservDbConn, _l: MaybeLoggedIn, h: String) -> Option<Redirect> {
    use crate::schema::users::dsl::*;

    let u: User = users
        .filter(handle.like(h))
        .first(&*conn)
        .optional()
        .expect("Failed to get user from database")?;

    Some(Redirect::to(format!("/users/{}", u.id)))
}

/// GET handler for '/users/<h>/edit'
/// gets the user template page for editing

#[get("/users/<h>/edit")]
pub fn user_edit(conn: ObservDbConn, l: UserGuard, h: i32) -> Option<EditUserTemplate> {
    use crate::schema::users::dsl::*;

    Some(EditUserTemplate {
        logged_in: Some(l.0),
        user: users
            .find(h)
            .first(&*conn)
            .optional()
            .expect("Failed to get user from database")?,
    })
}

/// PUT handler for '/users/<h>'
/// Puts up the new changes made in the user edit and changes the users data

#[put("/users/<h>", data = "<edituser>")]
pub fn user_edit_put(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    edituser: Form<NewUser>,
) -> Result<Redirect, Status> {
    let l = l.0;
    let mut edituser = edituser.into_inner();

    use crate::schema::users::dsl::*;
    // Get some more info about the edited user
    let (esalt, phash, etier) = users
        .find(h)
        .select((salt, password_hash, tier))
        .first(&*conn)
        .expect("Failed to get user from database");

    if l.tier > 1 || l.id == h {
        if edituser.password_hash.is_empty() {
            edituser.salt = esalt;
            edituser.password_hash = phash;
        } else {
            let (phash, psalt) = hash_password(edituser.password_hash);
            edituser.password_hash = phash;
            edituser.salt = psalt;
        }

        // if the logged in user can't change tiers
        // of if it's the admin user
        // don't change tiers
        if l.tier <= 1 || h == 0 {
            edituser.tier = etier;
        }

        update(users.find(h))
            .set(&edituser)
            .execute(&*conn)
            .expect("Failed to update user in database");

        Ok(Redirect::to(format!("/users/{}", edituser.handle)))
    } else {
        Err(Status::Unauthorized)
    }
}

/// DELETE handler for '/users/<h>'
/// delets all user data from the database

#[delete("/users/<h>")]
pub fn user_delete(conn: ObservDbConn, _l: AdminGuard, h: i32) -> Redirect {
    use crate::schema::users::dsl::*;
    delete(users.find(h))
        .execute(&*conn)
        .expect("Failed to delete user from database");

    Redirect::to("/users")
}

/// GET handler for '/users?<s>'
/// Return a list of users form a search string

#[get("/users?<s>")]
pub fn users(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>) -> UsersListTemplate {
    UsersListTemplate {
        logged_in: l.user(),
        users: filter_users(&*conn, s),
    }
}

/// GET handler for 'users.json?<s>'
/// Returns the JSON object for a user with an optional search string

#[get("/users.json?<s>")]
pub fn users_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<User>> {
    Json(filter_users(&*conn, s))
}

///HELPER FUNCTIONS

///filter_users takes in string into the search bar breaks it down and brings back a list of users that it matches

pub fn filter_users(conn: &SqliteConnection, term: Option<String>) -> Vec<User> {
    use crate::schema::users::dsl::*;

    if let Some(term) = term {
        let sterm = format!("%{}%", term);
        let email_term = format!("%{}@", term);
        let filter = real_name
            .like(&sterm)
            .or(email.like(&email_term))
            .or(handle.like(&sterm));
        users.filter(filter).load(conn)
    } else {
        users.load(conn)
    }
    .expect("Failed to get users")
}

/// finds the project that the user is related to
use crate::models::{Project, RelationProjectUser};
pub fn user_projects(conn: &SqliteConnection, user: &User) -> Vec<Project> {
    RelationProjectUser::belonging_to(user)
        .load::<RelationProjectUser>(conn)
        .expect("Failed to load relations from database")
        .iter()
        .map(|r| {
            use crate::schema::projects::dsl::*;
            projects
                .find(r.project_id)
                .first(conn)
                .expect("Failed to load project from database")
        })
        .collect()
}

/// finds a group the user is a part of
use crate::models::{Group, RelationGroupUser};
pub fn user_groups(conn: &SqliteConnection, user: &User) -> Vec<Group> {
    RelationGroupUser::belonging_to(user)
        .load::<RelationGroupUser>(conn)
        .expect("Failed to get relations from database")
        .iter()
        .map(|r| {
            use crate::schema::groups::dsl::*;
            groups
                .find(r.group_id)
                .first(conn)
                .expect("Failed to get group from database")
        })
        .collect()
}

///Calculates a users grade bassed on attendence and total commits

pub fn grade_summary(conn: &SqliteConnection, user: &User) -> GradeSummary {
    use crate::models::Attendable;
    use crate::models::Attendance;

    let at = Attendance::belonging_to(user)
        .load::<Attendance>(conn)
        .expect("Failed to load attendance from database")
        .iter()
        .map(|a| {
            if a.is_event {
                use crate::models::Event;
                use crate::schema::events::dsl::*;
                Box::new(
                    events
                        .find(a.event_id.unwrap())
                        .first::<Event>(conn)
                        .expect("Failed to load event from database"),
                ) as Box<dyn Attendable>
            } else {
                use crate::models::Meeting;
                use crate::schema::meetings::dsl::*;
                Box::new(
                    meetings
                        .find(a.meeting_id.unwrap())
                        .first::<Meeting>(conn)
                        .expect("Failed to load meeting from database"),
                ) as Box<dyn Attendable>
            }
        })
        .collect();

    let nat: usize = user_groups(conn, user).iter().fold(0, |a, g| {
        use crate::schema::meetings::dsl::*;
        a + meetings
            .filter(group_id.eq(g.id))
            .count()
            .get_result::<i64>(conn)
            .expect("Failed to get a count of meetings") as usize
    });

    GradeSummary {
        attendances: at,
        needed_attendances: nat,
        commit_count: user_commits_count(conn, user),
    }
}

/// Counts the number of total commits user has made
use crate::handlers::project_commits;
pub fn user_commits_count(conn: &SqliteConnection, user: &User) -> Option<usize> {
    Some(
        user_projects(conn, &user)
            .iter()
            .filter_map(|p| project_commits(conn, p))
            .flatten()
            .collect::<Vec<serde_json::Value>>()
            .first()?
            .as_array()?
            .iter()
            .filter_map(|c| {
                if c.get("author")?.get("login")?.as_str()? == user.handle {
                    Some(c)
                } else {
                    None
                }
            })
            .count(),
    )
}
