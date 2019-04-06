use std::io::Cursor;
use std::path::PathBuf;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{ContentType, Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Redirect, Response};
use rocket::Request;
use rocket_contrib::json::Json;

use crate::guards::*;
use crate::helpers::*;
use crate::models;
use crate::templates::*;
use crate::ObservDbConn;

#[get("/")]
pub fn index(l: MaybeLoggedIn) -> IndexTemplate {
    IndexTemplate {
        logged_in: l.user(),
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Embed;

#[get("/static/<file..>")]
pub fn staticfile(file: PathBuf) -> Option<Response<'static>> {
    let ctype = ContentType::from_extension(file.extension()?.to_str().unwrap())?;
    let bytes = Cursor::new(Embed::get(file.to_str().unwrap())?);

    Some(Response::build().header(ctype).sized_body(bytes).finalize())
}

#[get("/calendar")]
pub fn calendar(conn: ObservDbConn, l: MaybeLoggedIn) -> CalendarTemplate {
    use crate::schema::events::dsl::*;

    CalendarTemplate {
        logged_in: l.user(),
        events: events.load(&conn.0).expect("Failed to get events"),
    }
}

#[get("/signup")]
pub fn signup() -> SignUpTemplate {
    SignUpTemplate
}

#[post("/signup", data = "<newuser>")]
pub fn signup_post(
    conn: ObservDbConn,
    mut cookies: Cookies,
    newuser: Form<models::NewUser>,
) -> Redirect {
    use crate::schema::users::dsl::*;

    let mut newuser = newuser.into_inner();
    let newsalt = gen_salt();
    newuser.salt = newsalt.clone();
    newuser.password_hash = hash_password(newuser.password_hash, &newsalt);

    insert_into(users)
        .values(&newuser)
        .execute(&*conn)
        .expect("Failed to add user to database");

    let user: models::User = users
        .filter(&email.eq(newuser.email))
        .first(&*conn)
        .expect("Failed to get user from database");

    cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));

    Redirect::to(format!("/u/{}", user.handle))
}

#[get("/login")]
pub fn login() -> LogInTemplate {
    LogInTemplate
}

#[post("/login", data = "<creds>")]
pub fn login_post(
    conn: ObservDbConn,
    mut cookies: Cookies,
    creds: Form<models::LogInForm>,
) -> Redirect {
    use crate::schema::users::dsl::*;

    let creds = creds.into_inner();

    let user: models::User = users
        .filter(&email.eq(creds.email))
        .first(&*conn)
        .expect("Failed to get user from database");

    if verify_password(creds.password, user.password_hash, &user.salt) {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

#[get("/u/<h>")]
pub fn user(conn: ObservDbConn, l: MaybeLoggedIn, h: String) -> UserTemplate {
    use crate::schema::users::dsl::*;

    UserTemplate {
        logged_in: l.user(),
        user: users
            .filter(handle.like(h))
            .first(&*conn)
            .expect("Failed to get user from database"),
    }
}

#[get("/users?<s>")]
pub fn users(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>) -> UsersListTemplate {
    UsersListTemplate {
        logged_in: l.user(),
        users: filter_users(&*conn, s),
    }
}

#[get("/users.json?<s>")]
pub fn users_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<models::User>> {
    Json(filter_users(&*conn, s))
}

#[get("/projects?<s>")]
pub fn projects(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>) -> ProjectsListTemplate {
    ProjectsListTemplate {
        logged_in: l.user(),
        projects: filter_projects(&*conn, s),
    }
}

#[get("/projects.json?<s>")]
pub fn projects_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<models::Project>> {
    Json(filter_projects(&*conn, s))
}

#[get("/p/<n>")]
pub fn project(conn: ObservDbConn, l: MaybeLoggedIn, n: String) -> Option<ProjectTemplate> {
    use crate::schema::projects::dsl::*;

    let p: models::Project = projects
        .filter(name.like(n))
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;
    
    let r: Vec<models::Repo> = models::Repo::belonging_to(&p)
        .load(&*conn)
        .expect("Failed to get project's repos from database");

    Some(ProjectTemplate {
        logged_in: l.user(),
        project: p,
        repos: r,
    })
}

#[get("/calendar/newevent")]
pub fn newevent(admin: AdminGuard) -> NewEventTemplate {
    NewEventTemplate {
        logged_in: Some(admin.0),
    }
}

#[post("/calendar/newevent", data = "<newevent>")]
pub fn newevent_post(
    conn: ObservDbConn,
    _admin: AdminGuard,
    newevent: Form<models::NewEvent>,
) -> Redirect {
    use crate::schema::events::dsl::*;

    insert_into(events)
        .values(&newevent.0)
        .execute(&*conn)
        .expect("Failed to add user to database");

    Redirect::to("/calendar")
}

//# Catchers

#[catch(401)]
pub fn catch_401() -> Redirect {
    Redirect::to("/login")
}

#[catch(403)]
pub fn catch_403(req: &Request) -> Error403Template {
    let l = req.guard::<MaybeLoggedIn>().unwrap();
    Error403Template {
        logged_in: l.user(),
    }
}

#[catch(404)]
pub fn catch_404(req: &Request) -> Error404Template {
    let l = req.guard::<MaybeLoggedIn>().unwrap();
    Error404Template {
        logged_in: l.user(),
    }
}
