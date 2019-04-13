use std::io::Cursor;
use std::path::PathBuf;

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::{ContentType, Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Redirect, Response};
use rocket::Request;
use rocket_contrib::json::Json;
use serde_json;

use crate::guards::*;
use crate::helpers::*;
use crate::models::*;
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
pub fn dashboard(l: UserGuard) -> DashboardTemplate {
    DashboardTemplate {
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

#[get("/signup")]
pub fn signup(l: MaybeLoggedIn) -> SignUpTemplate {
    SignUpTemplate {
        logged_in: l.user(),
    }
}

//# Sign Up and Log In Handlers

#[post("/signup", data = "<newuser>")]
pub fn signup_post(conn: ObservDbConn, mut cookies: Cookies, newuser: Form<NewUser>) -> Redirect {
    use crate::schema::users::dsl::*;

    let mut newuser = newuser.into_inner();
    let newsalt = gen_salt();
    newuser.salt = newsalt.clone();
    newuser.password_hash = hash_password(newuser.password_hash, &newsalt);
    newuser.tier = 0;

    insert_into(users)
        .values(&newuser)
        .execute(&*conn)
        .expect("Failed to add user to database");

    let user: User = users
        .filter(&email.eq(newuser.email))
        .first(&*conn)
        .expect("Failed to get user from database");

    {
        use crate::schema::relation_group_user::dsl::*;
        insert_into(relation_group_user)
            .values(&NewRelationGroupUser {
                group_id: 0,
                user_id: user.id,
            })
            .execute(&*conn)
            .expect("Failed to insert new relation into database");
    }

    cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));

    Redirect::to(format!("/users/{}", user.handle))
}

#[get("/login")]
pub fn login(l: MaybeLoggedIn) -> LogInTemplate {
    LogInTemplate {
        logged_in: l.user(),
    }
}

#[derive(Default, FromForm)]
pub struct LogInForm {
    pub email: String,
    pub password: String,
}

#[post("/login?<to>", data = "<creds>")]
pub fn login_post(
    conn: ObservDbConn,
    mut cookies: Cookies,
    creds: Form<LogInForm>,
    to: Option<String>,
) -> Redirect {
    use crate::schema::users::dsl::*;

    let creds = creds.into_inner();

    let to = to.unwrap_or(String::from("/"));

    let user: User = users
        .filter(&email.eq(creds.email))
        .first(&*conn)
        .expect("Failed to get user from database");

    if verify_password(creds.password, user.password_hash, &user.salt) {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        Redirect::to(to)
    } else {
        Redirect::to(format!("/login?to={}", to))
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

//# User Handlers

#[get("/users/<h>")]
pub fn user(conn: ObservDbConn, l: MaybeLoggedIn, h: String) -> Option<UserTemplate> {
    use crate::schema::users::dsl::*;

    Some(UserTemplate {
        logged_in: l.user(),
        user: users
            .filter(handle.like(h))
            .first(&*conn)
            .optional()
            .expect("Failed to get user from database")?,
    })
}

#[get("/users/<h>", rank = 2)]
pub fn user_by_id(conn: ObservDbConn, l: MaybeLoggedIn, h: i32) -> Option<Redirect> {
    use crate::schema::users::dsl::*;

    let u: User = users
        .find(h)
        .first(&*conn)
        .optional()
        .expect("Failed to get user from database")?;

    Some(Redirect::to(format!("/users/{}", u.handle)))
}

#[get("/users/<h>")]
pub fn edituser(conn: ObservDbConn, l: UserGuard, h: String) -> Option<EditUserTemplate> {
    use crate::schema::users::dsl::*;

    Some(EditUserTemplate {
        logged_in: Some(l.0),
        user: users
            .filter(handle.like(h))
            .first(&*conn)
            .optional()
            .expect("Failed to get user from database")?,
    })
}

#[put("/users/<h>", data = "<edituser>")]
pub fn edituser_put(
    conn: ObservDbConn,
    l: UserGuard,
    h: String,
    edituser: Form<NewUser>,
) -> Redirect {
    let l = l.0;
    let mut edituser = edituser.into_inner();

    if !l.tier > 1 {
        edituser.tier = 0;
    }

    use crate::schema::users::dsl::*;
    update(users)
        .set(&edituser)
        .execute(&*conn)
        .expect("Failed to update user in database");

    Redirect::to(format!("/users/{}", edituser.handle))
}

#[delete("/users/<h>")]
pub fn user_delete(conn: ObservDbConn, l: AdminGuard, h: String) -> Redirect {
    use crate::schema::users::dsl::*;
    delete(users.filter(handle.eq(h)))
        .execute(&*conn)
        .expect("Failed to delete user from database");
    Redirect::to("/users")
}

#[get("/users?<s>")]
pub fn users(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>) -> UsersListTemplate {
    UsersListTemplate {
        logged_in: l.user(),
        users: filter_users(&*conn, s),
    }
}

#[get("/users.json?<s>")]
pub fn users_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<User>> {
    Json(filter_users(&*conn, s))
}

//# Project Handlers

#[get("/projects/<n>")]
pub fn project(conn: ObservDbConn, l: MaybeLoggedIn, n: String) -> Option<ProjectTemplate> {
    use crate::schema::projects::dsl::*;

    let p: Project = projects
        .filter(name.like(n))
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;

    Some(ProjectTemplate {
        logged_in: l.user(),
        repos: serde_json::from_str(&p.repos).unwrap(),
        project: p,
    })
}

#[get("/projects/<n>", rank = 2)]
pub fn project_by_id(conn: ObservDbConn, l: MaybeLoggedIn, n: i32) -> Option<Redirect> {
    use crate::schema::projects::dsl::*;
    let p: Project = projects
        .find(n)
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;

    Some(Redirect::to(format!("/projects/{}", p.name)))
}

#[get("/projects/new")]
pub fn newproject(l: UserGuard) -> NewProjectTemplate {
    NewProjectTemplate {
        logged_in: Some(l.0),
    }
}

#[post("/projects/new", data = "<newproject>")]
pub fn newproject_post(conn: ObservDbConn, l: UserGuard, newproject: Form<NewProject>) -> Redirect {
    let mut newproject = newproject.into_inner();
    newproject.owner_id = l.0.id;

    use crate::schema::projects::dsl::*;
    insert_into(projects)
        .values(&newproject)
        .execute(&*conn)
        .expect("Failed to insert project into database");

    let p: Project = projects
        .filter(name.eq(newproject.name))
        .first(&*conn)
        .expect("Failed to get project from database");

    use crate::schema::relation_project_user::dsl::*;
    insert_into(relation_project_user)
        .values(&NewRelationProjectUser {
            project_id: p.id,
            user_id: l.0.id,
        })
        .execute(&*conn)
        .expect("Failed to add user to project");

    Redirect::to(format!("/projects/{}", p.name))
}

#[get("/projects/<h>")]
pub fn editproject(l: UserGuard, h: String) -> Option<EditProjectTemplate> {
    unimplemented!()
}

#[put("/projects/<h>", data = "<editproject>")]
pub fn editproject_put(conn: ObservDbConn, l: UserGuard, h: String, editproject: Form<NewProject>) {
    unimplemented!()
}

#[delete("/projects/<h>")]
pub fn project_delete(conn: ObservDbConn, l: AdminGuard, h: String) -> Redirect {
    unimplemented!()
}

#[get("/projects?<s>")]
pub fn projects(conn: ObservDbConn, l: MaybeLoggedIn, s: Option<String>) -> ProjectsListTemplate {
    ProjectsListTemplate {
        logged_in: l.user(),
        projects: filter_projects(&*conn, s),
    }
}

#[get("/projects.json?<s>")]
pub fn projects_json(conn: ObservDbConn, s: Option<String>) -> Json<Vec<Project>> {
    Json(filter_projects(&*conn, s))
}

//# Calendar Handlers

#[get("/calendar")]
pub fn calendar(conn: ObservDbConn, l: MaybeLoggedIn) -> CalendarTemplate {
    use crate::schema::events::dsl::*;

    CalendarTemplate {
        logged_in: l.user(),
        events: events.load(&conn.0).expect("Failed to get events"),
    }
}

#[get("/calendar.json")]
pub fn calendar_json(conn: ObservDbConn) -> Json<Vec<Event>> {
    use crate::schema::events::dsl::*;

    Json(events.load(&*conn).expect("Failed to get events"))
}

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
            .expect("Failed to get users from the database"),
    })
}

#[put("/calendar/<eid>", data = "<editevent>")]
pub fn editevent_put(
    conn: ObservDbConn,
    l: AdminGuard,
    eid: i32,
    editevent: Form<NewEvent>,
) -> Redirect {
    use crate::schema::events::dsl::*;

    let mut editevent = editevent.into_inner();
    editevent.code = events.find(eid).select(code).first(&*conn).expect("Failed to get event code");

    update(events.find(eid))
        .set(&editevent)
        .execute(&*conn)
        .expect("Failed to update event in database");
    Redirect::to("/calendar")
}

#[delete("/calendar/<eid>")]
pub fn event_delete(conn: ObservDbConn, l: AdminGuard, eid: i32) -> Redirect {
    use crate::schema::events::dsl::*;
    delete(events.find(eid))
        .execute(&*conn)
        .expect("Failed to delete event from database");
    Redirect::to("/calendar")
}

#[get("/calendar/new")]
pub fn newevent(conn: ObservDbConn, admin: AdminGuard) -> NewEventTemplate {
    use crate::schema::users::dsl::*;
    NewEventTemplate {
        logged_in: Some(admin.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from the database"),
    }
}

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

//# Groups and Meetings

#[get("/groups/<gid>")]
pub fn group(conn: ObservDbConn, l: UserGuard, gid: i32) -> Option<GroupTemplate> {
    use crate::schema::groups::dsl::*;

    let g: Group = groups
        .find(gid)
        .first(&*conn)
        .optional()
        .expect("Failed to get groups from the database")?;

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
            .expect("Failed to get groups from the database"),
    }
}

#[get("/groups/new")]
pub fn newgroup(l: AdminGuard) -> NewGroupTemplate {
    NewGroupTemplate {
        logged_in: Some(l.0),
    }
}

#[post("/groups/new", data = "<newgroup>")]
pub fn newgroup_post(conn: ObservDbConn, l: AdminGuard, newgroup: Form<NewGroup>) -> Redirect {
    unimplemented!()
}

#[post("/groups/<gid>", data = "<newmeeting>")]
pub fn newmeeting_post(
    conn: ObservDbConn,
    l: MentorGuard,
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

//# Attendance

#[get("/attend")]
pub fn attend(l: UserGuard) -> AttendTemplate {
    AttendTemplate {
        logged_in: Some(l.0),
    }
}

#[derive(FromForm)]
pub struct AttendCode {
    code: String,
}

#[post("/attend", data = "<code>")]
pub fn attend_post(conn: ObservDbConn, l: UserGuard, code: Form<AttendCode>) -> Redirect {
    use crate::schema::attendances::dsl::*;

    if let Some(m) = verify_code(&*conn, &code.code) {
        let (mid, eid) = if m.is_event() {
            (None, Some(m.id()))
        } else {
            (Some(m.id()), None)
        };
        let newattend = NewAttendance {
            user_id: l.0.id,
            is_event: m.is_event(),
            meeting_id: mid,
            event_id: eid,
        };
        insert_into(attendances)
            .values(&newattend)
            .execute(&*conn)
            .expect("Failed to insert attendance into database");
        Redirect::to("/")
    } else {
        Redirect::to("/attend")
    }
}

//# News

#[get("/news")]
pub fn news(conn: ObservDbConn, l: MaybeLoggedIn) -> NewsTemplate {
    use crate::schema::news::dsl::*;
    NewsTemplate {
        logged_in: l.user(),
        news: news.load(&*conn).expect("Failed to get news from database"),
    }
}

#[get("/news.json")]
pub fn news_json(conn: ObservDbConn, l: MaybeLoggedIn) -> Json<Vec<NewsEvent>> {
    use crate::schema::news::dsl::*;
    Json(news.load(&*conn).expect("Failed to get news from database"))
}

#[get("/news/<nid>")]
pub fn newsevent(conn: ObservDbConn, l: MaybeLoggedIn, nid: i32) -> NewsEventTemplate {
    use crate::schema::news::dsl::*;
    NewsEventTemplate {
        logged_in: l.user(),
        newsevent: news
            .find(nid)
            .first(&*conn)
            .expect("Failed to get news event from database"),
    }
}

#[get("/news/new")]
pub fn newnewsevent(conn: ObservDbConn, l: MaybeLoggedIn) -> NewNewsEventTemplate {
    unimplemented!()
}

#[post("/news/new", data = "<newnewsevent>")]
pub fn newnewsevent_post(
    conn: ObservDbConn,
    l: MaybeLoggedIn,
    newnewsevent: Form<NewNewsEvent>,
) -> NewNewsEventTemplate {
    unimplemented!()
}

#[get("/news/new")]
pub fn editnewsevent(conn: ObservDbConn, l: MaybeLoggedIn) -> NewNewsEventTemplate {
    unimplemented!()
}

#[put("/news/new", data = "<newnewsevent>")]
pub fn editnewsevent_post(
    conn: ObservDbConn,
    l: MaybeLoggedIn,
    newnewsevent: Form<NewNewsEvent>,
) -> NewNewsEventTemplate {
    unimplemented!()
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
