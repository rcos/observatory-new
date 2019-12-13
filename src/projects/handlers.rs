use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::json::Json;
use serde_json;

use crate::guards::*;
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;
use crate::templates::{is_reserved, FormError};

/// GET handler for `/projects?s`
/// Project list page with an optional search string,

#[get("/projects?<s>&<a>")]
pub fn projects(
    conn: ObservDbConn,
    l: MaybeLoggedIn,
    s: Option<String>,
    a: Option<bool>,
) -> ProjectsListTemplate {
    ProjectsListTemplate {
        logged_in: l.user(),
        search_term: s.clone().unwrap_or_else(String::new),
        projects: filter_projects(&*conn, s, a),
        inactive: a.unwrap_or(false),
    }
}

/// GET handler for `/projects?s`
/// Return JSON object of the project with an optional search string

#[get("/projects.json?<s>&<a>")]
pub fn projects_json(conn: ObservDbConn, s: Option<String>, a: Option<bool>) -> Json<Vec<Project>> {
    Json(filter_projects(&*conn, s, a))
}

/// GET handler for `/projects/id`
/// Gets an indivual project from the data base by its ID and returns its template

#[get("/projects/<n>")]
pub fn project(conn: ObservDbConn, l: MaybeLoggedIn, n: i32) -> Option<ProjectTemplate> {
    use crate::schema::projects::dsl::*;
    use std::collections::HashMap;

    let p: Project = projects
        .find(n)
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;

    let rc = project_commits(&conn, &p)
        .unwrap_or(Vec::new())
        .iter()
        .enumerate()
        .map(|(i, repo)| {
            (
                project_repos(&p)[i]
                    .to_owned()
                    .replace("https://github.com/", ""),
                repo.as_array()
                    .unwrap_or(&Vec::new())
                    .into_iter()
                    .take(10)
                    .map(|commit| {
                        let auth_name = serde_json::to_string(&commit["commit"]["author"]["name"])
                            .unwrap()
                            .replace("\"", "");
                        let auth_email =
                            serde_json::to_string(&commit["commit"]["author"]["email"])
                                .unwrap()
                                .replace("\"", "");
                        let full_msg = serde_json::to_string(&commit["commit"]["message"])
                            .unwrap()
                            .replace("\"", "");
                        let auth_url = serde_json::to_string(&commit["html_url"])
                            .unwrap()
                            .replace("\"", "");

                        let trunc_msg = String::from(
                            *full_msg
                                .split("\\n")
                                .collect::<Vec<&str>>()
                                .first()
                                .unwrap(),
                        );

                        (auth_name, auth_email, trunc_msg, auth_url)
                    })
                    .collect::<Vec<(String, String, String, String)>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    Some(ProjectTemplate {
        logged_in: l.user(),
        repos: project_repos(&p),
        users: project_users(&*conn, &p),
        project: p,
        recent_commits: rc,
    })
}

/// GET handler for `/projects/name`
/// Gets the ID of the project by its name and then redirects to the above function

#[get("/projects/<n>", rank = 2)]
pub fn project_by_handle(conn: ObservDbConn, _l: MaybeLoggedIn, n: String) -> Option<Redirect> {
    use crate::schema::projects::dsl::*;
    let p: Project = projects
        .filter(name.like(n))
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;

    Some(Redirect::to(format!("/projects/{}", p.id)))
}

/// GET handler for `/projects/new`
/// Returns the new project template

#[get("/projects/new?<e>")]
pub fn project_new(l: UserGuard, e: Option<FormError>) -> NewProjectTemplate {
    NewProjectTemplate {
        logged_in: Some(l.0),
        error: e,
    }
}

/// POST `/project/new`
/// Accepts the data from the new project template form and creates the project in the data base

#[post("/projects/new", data = "<newproject>")]
pub fn project_new_post(
    conn: ObservDbConn,
    l: UserGuard,
    newproject: Form<NewProject>,
) -> Redirect {
    let mut newproject = newproject.into_inner();
    newproject.name.truncate(50); // sets a character limit for a new project name
    newproject.description.truncate(500); // sets a character limit for a new project description
    newproject.repos.truncate(100); // sets a character limit for a new repository URL
    newproject.owner_id = l.0.id; // set owner to be the person who created the project
    newproject.active = true;

    if let Err(e) = is_reserved(&newproject.name) {
        return Redirect::to(format!("/projects/new?e={}", e));
    }

    // handles the fact that projects can have multiple repos
    newproject.repos = serde_json::to_string(
        &serde_json::from_str::<Vec<String>>(&newproject.repos)
            .unwrap()
            .iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&String>>(),
    )
    .unwrap();

    // inserts the project into the database
    use crate::schema::projects::dsl::*;
    use diesel::result::DatabaseErrorKind;
    use diesel::result::Error;
    match insert_into(projects).values(&newproject).execute(&*conn) {
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            return Redirect::to(format!("/projects/new?e={}", FormError::TakenName))
        }
        Err(_) => return Redirect::to(format!("/projects/new?e={}", FormError::Other)),
        Ok(_) => ()
    }

    // retrieves the object from the database after creating it
    let p: Project = projects
        .filter(name.eq(newproject.name)) // CHANGEME switch to id
        .first(&*conn)
        .expect("Failed to get project from database");

    //creates the relation for the project owner
    use crate::schema::relation_project_user::dsl::*;
    insert_into(relation_project_user)
        .values(&NewRelationProjectUser {
            project_id: p.id,
            user_id: l.0.id,
        })
        .execute(&*conn)
        .expect("Failed to add user to project");

    Redirect::to(format!("/projects/{}", p.id))
}

/// GET handler for `/projects/edit`
/// Get the project template for editing

#[get("/projects/<h>/edit?<e>")]
pub fn project_edit(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    e: Option<FormError>,
) -> Result<EditProjectTemplate, Status> {
    use crate::schema::projects::dsl::*;
    use crate::schema::users::dsl::*;

    let p: Project = projects
        .find(h)
        .first(&*conn)
        .expect("Failed to get project from database");

    //checks to see what tier logged in user is or if there the owner
    if l.0.tier > 1 || p.owner_id == l.0.id {
        Ok(EditProjectTemplate {
            logged_in: Some(l.0),
            repos: project_repos(&p),
            project: p,
            all_users: users
                .load(&*conn)
                .expect("Failed to get users from database"),
            error: e,
        })
    } else {
        Err(Status::Unauthorized)
    }
}

/// PUT handler for `/projects/edit`
/// Uploads the edits made to the projects template form

#[put("/projects/<h>", data = "<editproject>")]
pub fn project_edit_put(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    editproject: Form<NewProject>,
) -> Result<Redirect, Status> {
    use crate::schema::projects::dsl::*;

    let mut editproject = editproject.into_inner();
    editproject.name.truncate(50); // sets a character limit for an edited project name
    editproject.description.truncate(500); // sets a character limit for a new project name
    editproject.repos.truncate(100); // sets a character limit for an edited repository URL

    editproject.repos = serde_json::to_string(
        &serde_json::from_str::<Vec<String>>(&editproject.repos)
            .unwrap()
            .iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&String>>(),
    )
    .unwrap();

    let p: Project = projects
        .find(h)
        .first(&*conn)
        .expect("Failed to get project from database");

    //checks to see what tier logged in user is or if there the owner so no one outside the project messes with it
    if l.0.tier > 1 || p.owner_id == l.0.id {
        if let Err(e) = is_reserved(&editproject.name) {
            return Ok(Redirect::to(format!("/projects/{}/edit?e={}", h, e)));
        }

        update(projects.find(h))
            .set(&editproject)
            .execute(&*conn)
            .expect("Failed to update project in database");
        Ok(Redirect::to(format!("/projects/{}", h)))
    } else {
        Err(Status::Unauthorized)
    }
}

/// DELETE handler for `/projects/h`
///
/// Deletes relation from all users tied to the project then deletes the project

#[delete("/projects/<h>")]
pub fn project_delete(conn: ObservDbConn, l: UserGuard, h: i32) -> Result<Redirect, Status> {
    use crate::schema::projects::dsl::*;
    // Find the project
    let p: Project = projects
        .find(h)
        .first(&*conn)
        .expect("Failed to get project from database");

    // If they are an admin or the project owner
    if l.0.tier > 1 || p.owner_id == l.0.id {
        // Delete the relations
        use crate::schema::relation_project_user::dsl::*;
        delete(relation_project_user.filter(project_id.eq(h)))
            .execute(&*conn)
            .expect("Failed to delete relations from database");

        // Delete the project
        delete(projects.find(h))
            .execute(&*conn)
            .expect("Failed to delete project from database");
        Ok(Redirect::to("/projects"))
    } else {
        Err(Status::Unauthorized)
    }
}

/// GET handler for `/projects/h/members`
/// Redirects to the projects page

#[get("/projects/<h>/members")]
pub fn project_members(h: i32) -> Redirect {
    Redirect::to(format!("/projects/{}", h))
}

/// GET handler for `/projects/h/members.json`
/// Returns the JSON object of the members of the project

#[get("/projects/<h>/members.json")]
pub fn project_members_json(conn: ObservDbConn, h: i32) -> Json<Vec<User>> {
    Json(project_users(&*conn, &{
        use crate::schema::projects::dsl::*;
        projects
            .find(h)
            .first(&*conn)
            .expect("Failed to get project from database")
    }))
}

/// GET handler for `/projects/h/members/add`
/// Returns the member add page

#[get("/projects/<h>/members/add")]
pub fn project_member_add(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
) -> Result<AddUserTemplate, Status> {
    let p: Project = {
        use crate::schema::projects::dsl::*;
        projects
            .find(h)
            .first(&*conn)
            .expect("Failed to get project from database")
    };

    let pu = project_users(&*conn, &p);

    use crate::schema::users::dsl::*;

    //checks to see what tier your logged into
    if l.0.tier > 0 || l.0.id == p.owner_id {
        Ok(AddUserTemplate {
            logged_in: Some(l.0),
            project: p,
            all_users: {
                // gets a list of users not in the project
                users
                    .filter(id.ne(0))
                    .load(&*conn)
                    .expect("Failed to get users from database")
                    .iter()
                    .filter(|&e| !pu.contains(e))
                    .cloned()
                    .collect()
            },
        })
    } else {
        Err(Status::Unauthorized)
    }
}

///Form struct for user add

#[derive(FromForm)]
pub struct UserId {
    pub uid: i32,
}

///POST handler for `projects/h/members/add`
/// This adds the user to the project form

#[post("/projects/<h>/members/add", data = "<userid>")]
pub fn project_member_add_post(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    userid: Form<UserId>,
) -> Result<Redirect, Status> {
    let p: Project = {
        use crate::schema::projects::dsl::*;
        projects
            .find(h)
            .first(&*conn)
            .expect("Failed to get project from database")
    };

    //checks to see if your the right tier so you cant jsut send what you want
    if l.0.tier > 0 || l.0.id == p.owner_id {
        use crate::schema::relation_project_user::dsl::*;
        insert_into(relation_project_user)
            .values(&NewRelationProjectUser {
                project_id: h,
                user_id: userid.into_inner().uid,
            })
            .execute(&*conn)
            .expect("Failed to insert relation into database");
        Ok(Redirect::to(format!("/projects/{}", h)))
    } else {
        Err(Status::Unauthorized)
    }
}

///DELETE handler for `projects/h/members/uid`
/// Removes user relation from the project

#[delete("/projects/<h>/members/<uid>")]
pub fn project_member_delete(
    conn: ObservDbConn,
    l: UserGuard,
    h: i32,
    uid: i32,
) -> Result<Redirect, Status> {
    let owner_id: i32 = {
        use crate::schema::projects::dsl::*;
        projects
            .find(h)
            .select(owner_id)
            .first(&*conn)
            .expect("Failed to get project from database")
    };

    if l.0.tier > 0 || l.0.id == owner_id {
        use crate::schema::relation_project_user::dsl::*;
        delete(relation_project_user.filter(project_id.eq(h).and(user_id.eq(uid))))
            .execute(&*conn)
            .expect("Failed to delete relation from database");
        Ok(Redirect::to(format!("/projects/{}", h)))
    } else {
        Err(Status::Unauthorized)
    }
}

///GET handler for `projects/h/members/join`
/// Returns the join page for a particular project

#[get("/projects/<h>/members/join")]
pub fn project_join(conn: ObservDbConn, l: UserGuard, h: i32) -> JoinTemplate {
    use crate::schema::projects::dsl::*;
    JoinTemplate {
        logged_in: Some(l.0),
        project: projects
            .find(h)
            .first(&*conn)
            .expect("Failed to get project from database"),
    }
}

///POST handler for `projects/h/members/join`
/// The User confirms they want to join the project user relation added to project database

#[post("/projects/<h>/members/join")]
pub fn project_join_post(conn: ObservDbConn, l: UserGuard, h: i32) -> Result<Redirect, Status> {
    use crate::schema::projects::dsl::*;

    let a: bool = projects
        .select(active)
        .find(h)
        .first(&*conn)
        .expect("Failed to get project from database");

    if a {
        use crate::schema::relation_project_user::dsl::*;
        insert_into(relation_project_user)
            .values(&NewRelationProjectUser {
                project_id: h,
                user_id: l.0.id,
            })
            .execute(&*conn)
            .expect("Failed to add relation to database");
        Ok(Redirect::to(format!("/projects/{}", h)))
    } else {
        Err(Status::Conflict)
    }
}

//# Helper Functions

pub fn project_repos(p: &Project) -> Vec<String> {
    serde_json::from_str(&p.repos).unwrap()
}

pub fn filter_projects(
    conn: &SqliteConnection,
    term: Option<String>,
    inact: Option<bool>,
) -> Vec<Project> {
    use crate::schema::projects::dsl::*;

    if let Some(term) = term {
        let sterm = format!("%{}%", term);
        let filter = name.like(&sterm);

        match inact {
            Some(true) => projects.filter(filter).load(conn),
            Some(false) | None => projects.filter(filter.and(active.eq(true))).load(conn),
        }
    } else {
        match inact {
            Some(true) => projects.load(conn),
            Some(false) | None => projects.filter(active.eq(true)).load(conn),
        }
    }
    .expect("Failed to get projects")
}

use crate::models::User;

pub fn project_users(conn: &SqliteConnection, project: &Project) -> Vec<User> {
    RelationProjectUser::belonging_to(project)
        .load::<RelationProjectUser>(conn)
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

/// Get the commits in the project
///
/// This function calls to the GitHub API to get the commits.
///
/// If the project does not use GitHub for it's repo this returns `None`.
/// Otherwise it returns a vector of JSON values with each repo having an
/// entry.
///
/// TODO support other services like GitLab.
pub fn project_commits(conn: &SqliteConnection, proj: &Project) -> Option<Vec<serde_json::Value>> {
    // Get the repos from the DB
    let mut repos: Vec<String> = {
        use crate::schema::projects::dsl::*;
        serde_json::from_str(
            &projects
                .find(proj.id)
                .select(repos)
                .first::<String>(conn)
                .expect("Failed to get repos from the database"),
        )
        .unwrap()
    };

    // No repos at all
    if repos.is_empty() {
        return None;
    }

    // Use a regex to filter to only GitHub and convert to the API string
    use regex::Regex;
    let re = Regex::new(r"^(https?://)?github\.com/(\S+/\S+)/?$")
        .expect("Failed to build regular expression");
    repos = repos
        .iter()
        .filter(|s| re.is_match(&s))
        .map(|s| {
            String::from(re.replace(s, "https://api.github.com/repos/$2/commits?per_page=100"))
        })
        .collect();

    // If no GitHub repos
    if repos.is_empty() {
        return None;
    }

    // Get the commits and return them, filtering out errors
    Some(
        repos
            .iter()
            .filter_map(|s| {
                let res = reqwest::get(s);
                if res.is_ok() {
                    if let Ok(json) = res
                        .expect("Failed to get response from GitHub")
                        .json::<serde_json::Value>()
                    {
                        Some(json)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect(),
    )
}
