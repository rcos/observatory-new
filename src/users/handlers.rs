//!

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
use crate::templates::{is_reserved, FormError};

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

#[get("/users/<h>/edit?<e>")]
pub fn user_edit(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    e: Option<FormError>,
) -> Option<EditUserTemplate> {
    use crate::schema::users::dsl::*;

    Some(EditUserTemplate {
        logged_in: Some(l.0),
        user: users
            .find(h)
            .first(&*conn)
            .optional()
            .expect("Failed to get user from database")?,
        error: e,
    })
}

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
        if let Err(e) = is_reserved(&*edituser.handle) {
            return Ok(Redirect::to(format!("/users/{}/edit?e={}", h, e)));
        }

        // Check if user's email is already signed up
        if users
            .filter(&email.eq(&edituser.email).and(id.ne(h)))
            .first::<User>(&*conn)
            .optional()
            .expect("Failed to get user from database")
            .is_some()
        {
            return Ok(Redirect::to(format!(
                "/users/{}/edit?e={}",
                h,
                FormError::EmailExists
            )));
        }

        // Check if user's github is already signed up
        if users
            .filter(&handle.eq(&edituser.handle).and(id.ne(h)))
            .first::<User>(&*conn)
            .optional()
            .expect("Failed to get user from database")
            .is_some()
        {
            return Ok(Redirect::to(format!(
                "/users/{}/edit?e={}",
                h,
                FormError::GitExists
            )));
        }

        // Check if user's mattermost is already signed up
        if users
            .filter(&mmost.eq(&edituser.mmost).and(id.ne(h)))
            .first::<User>(&*conn)
            .optional()
            .expect("Failed to get user from database")
            .is_some()
        {
            return Ok(Redirect::to(format!(
                "/users/{}/edit?e={}",
                h,
                FormError::MmostExists
            )));
        }

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

#[delete("/users/<h>")]
pub fn user_delete(conn: ObservDbConn, _l: AdminGuard, h: i32) -> Redirect {
    // Delete the user
    use crate::schema::users::dsl::*;
    delete(users.find(h))
        .execute(&*conn)
        .expect("Failed to delete user from database");

    // Delete the relations to projects
    {
        use crate::schema::relation_project_user::dsl::*;
        delete(relation_project_user.filter(user_id.eq(h)))
            .execute(&*conn)
            .expect("Failed to delete relation from database");
    }

    // Delete the relations to groups
    {
        use crate::schema::relation_group_user::dsl::*;
        delete(relation_group_user.filter(user_id.eq(h)))
            .execute(&*conn)
            .expect("Failed to delete relation from database");
    }

    Redirect::to("/users")
}

#[get("/users?<s>&<a>")]
pub fn users(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>, a: Option<bool>) -> UsersListTemplate {
    UsersListTemplate {
        logged_in: l.user(),
        search_term: s.clone().unwrap_or_else(String::new),
        users: filter_users(&*conn, s, a),
        inactive: a.unwrap_or(false)
    }
}

#[get("/users.json?<s>&<a>")]
pub fn users_json(conn: ObservDbConn, s: Option<String>, a: Option<bool>) -> Json<Vec<User>> {
    Json(filter_users(&*conn, s, a))
}

pub fn filter_users(conn: &SqliteConnection, term: Option<String>, inact: Option<bool>) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let afilter = active.eq(true).and(former.eq(false));

    if let Some(term) = term {
        let sterm = format!("%{}%", term);
        let email_term = format!("%{}@", term);

        let filter = real_name
            .like(&sterm)
            .or(email.like(&email_term))
            .or(handle.like(&sterm));

        match inact {
            Some(true) => {
                users.filter(filter).load(conn)
            },
            Some(false) | None => {
                users.filter(filter.and(afilter)).load(conn)
            }
        }
    } else {
        match inact {
            Some(true) => {
                users.load(conn)
            },
            Some(false) | None => {
                users.filter(afilter).load(conn)
            }
        }
    }
    .expect("Failed to get users")
}

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
