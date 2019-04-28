use std::io::Cursor;
use std::path::PathBuf;

use rocket::http::ContentType;

use rocket::response::{Redirect, Response};
use rocket::Request;

use crate::guards::*;
use crate::templates::*;
use crate::ObservDbConn;

#[get("/")]
pub fn index(l: MaybeLoggedIn) -> IndexTemplate {
    IndexTemplate {
        logged_in: l.user(),
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[get("/dashboard")]
pub fn dashboard(conn: ObservDbConn, l: UserGuard) -> DashboardTemplate {
    use crate::users::handlers::{grade_summary, user_groups, user_projects};
    DashboardTemplate {
        summary: grade_summary(&*conn, &l.0),
        projects: user_projects(&*conn, &l.0),
        groups: user_groups(&*conn, &l.0),
        logged_in: Some(l.0),
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

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/favicon.webp")
}

//# Catchers

#[catch(401)]
pub fn catch_401(req: &Request) -> Redirect {
    Redirect::to(format!("/login?to={}", req.uri().path()))
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
