//! HTTP handlers for misc. pages
//!
//! These routes don't belong in any other category so are put here.

use std::io::Cursor;
use std::path::PathBuf;

use rocket::http::ContentType;

use rocket::response::{Redirect, Response};
use rocket::Request;

use crate::guards::*;
use crate::templates::*;
use crate::ObservDbConn;

// Load all the sub-module's handlers
pub use crate::attend::handlers::*;
pub use crate::auth::handlers::*;
pub use crate::calendar::handlers::*;
pub use crate::groups::handlers::*;
pub use crate::news::handlers::*;
pub use crate::projects::handlers::*;
pub use crate::users::handlers::*;

/// GET handler for `/`
///
/// The index page of the site
#[get("/")]
pub fn index(l: MaybeLoggedIn) -> IndexTemplate {
    IndexTemplate {
        logged_in: l.user(),
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[get("/big?<text>")]
pub fn big(l: MaybeLoggedIn, text: String) -> BigTemplate {
    BigTemplate {
        logged_in: l.user(),
        text,
    }
}

/// GET handler for `/dashboard`
///
/// The logged in user's dashboard showing their groups, projects, and attendance
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

// The access point for rust-embed.
// For some reason it doesn't like having doc-comments on it.
#[derive(RustEmbed)]
#[folder = "static/"]
struct Embed;

/// Paths that will not be served over `/static`
///
/// These files will not be served by the webserver.
/// However they are still embedded into the binary so be careful.
// Make sure to increment the length if you add something
const BLACKLIST: [&str; 1] = ["README.md"];

/// GET handler for static files
///
/// Any file in the `static/` directory can be accessed through this just
/// by specifying it's path.
///
/// These files are embedded into the binary at compile time and are always
/// available.
#[get("/static/<file..>")]
pub fn staticfile(file: PathBuf) -> Option<Response<'static>> {
    // If file is in the BLACKLIST return None
    if BLACKLIST.contains(&file.to_str().unwrap()) {
        None
    } else {
        // Get the mimetype from the request
        let ctype = ContentType::from_extension(file.extension()?.to_str().unwrap())?;
        // The bytes of the file in a Read and Seek-able Cursor
        let bytes = Cursor::new(Embed::get(file.to_str().unwrap())?);

        // Respond with the file
        Some(Response::build().header(ctype).sized_body(bytes).finalize())
    }
}

/// GET handler for `/favicon.ico`
///
/// Some browsers and utilities always look at `/favicon.ico` for the page
/// favicon, so this is a quick way to support that.
#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/img/favicon.webp")
}

//# # Error Catchers

/// Catch 401 errors
///
/// Redirects the user to the login page when they try to go to a page that
/// requires login.
#[catch(401)]
pub fn catch_401(req: &Request) -> Redirect {
    Redirect::to(format!("/login?to={}", req.uri().path()))
}

/// Catch 403 errors
///
/// A nice page for 403 errors when the user doesn't have access to the
/// page they are trying to visit.
#[catch(403)]
pub fn catch_403(req: &Request) -> Error403Template {
    let l = req.guard::<MaybeLoggedIn>().unwrap();
    Error403Template {
        logged_in: l.user(),
    }
}

/// Catch 404 errors
///
/// A nice page for 404 errors
#[catch(404)]
pub fn catch_404(req: &Request) -> Error404Template {
    let l = req.guard::<MaybeLoggedIn>().unwrap();
    Error404Template {
        logged_in: l.user(),
    }
}
