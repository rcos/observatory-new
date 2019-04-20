use std::io::Cursor;
use std::path::PathBuf;

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::{ContentType, Cookie, Cookies, Status};
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

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/favicon.webp")
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
    newuser.active = true;

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

    Redirect::to(format!("/users/{}", user.id))
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
pub fn user(conn: ObservDbConn, l: MaybeLoggedIn, h: i32) -> Option<UserTemplate> {
    use crate::schema::users::dsl::*;

    Some(UserTemplate {
        logged_in: l.user(),
        user: users
            .find(h)
            .first(&*conn)
            .optional()
            .expect("Failed to get user from database")?,
    })
}

#[get("/users/<h>", rank = 2)]
pub fn user_by_handle(conn: ObservDbConn, l: MaybeLoggedIn, h: String) -> Option<Redirect> {
    use crate::schema::users::dsl::*;

    let u: User = users
        .filter(handle.like(h))
        .first(&*conn)
        .optional()
        .expect("Failed to get user from database")?;

    Some(Redirect::to(format!("/users/{}", u.id)))
}

#[get("/users/<h>/edit")]
pub fn edituser(conn: ObservDbConn, l: UserGuard, h: i32) -> Option<EditUserTemplate> {
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

#[put("/users/<h>", data = "<edituser>")]
pub fn edituser_put(
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
            edituser.salt = gen_salt();
            edituser.password_hash = hash_password(edituser.password_hash, &edituser.salt);
        }

        // if the logged in user can't change tiers
        // of if it's the admin user
        // don't change tiers
        if !(l.tier > 1) || h == 0 {
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
pub fn user_delete(conn: ObservDbConn, l: AdminGuard, h: i32) -> Redirect {
    use crate::schema::users::dsl::*;
    delete(users.find(h))
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
pub fn project(conn: ObservDbConn, l: MaybeLoggedIn, n: i32) -> Option<ProjectTemplate> {
    use crate::schema::projects::dsl::*;

    let p: Project = projects
        .find(n)
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
pub fn project_by_handle(conn: ObservDbConn, l: MaybeLoggedIn, n: String) -> Option<Redirect> {
    use crate::schema::projects::dsl::*;
    let p: Project = projects
        .filter(name.like(n))
        .first(&*conn)
        .optional()
        .expect("Failed to get project from database")?;

    Some(Redirect::to(format!("/projects/{}", p.id)))
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

    Redirect::to(format!("/projects/{}", p.id))
}

#[get("/projects/<h>")]
pub fn editproject(l: UserGuard, h: i32) -> Option<EditProjectTemplate> {
    unimplemented!()
}

#[put("/projects/<h>", data = "<editproject>")]
pub fn editproject_put(conn: ObservDbConn, l: UserGuard, h: i32, editproject: Form<NewProject>) {
    unimplemented!()
}

#[delete("/projects/<h>")]
pub fn project_delete(conn: ObservDbConn, l: AdminGuard, h: i32) -> Redirect {
    use crate::schema::projects::dsl::*;
    delete(projects.find(h))
        .execute(&*conn)
        .expect("Failed to delete project from database");
    Redirect::to("/projects")
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
        events: events
            .order(start.asc())
            .load(&conn.0)
            .expect("Failed to get events"),
    }
}

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
pub fn newgroup(conn: ObservDbConn, l: AdminGuard) -> NewGroupTemplate {
    use crate::schema::users::dsl::*;
    NewGroupTemplate {
        logged_in: Some(l.0),
        all_users: users
            .load(&*conn)
            .expect("Failed to get users from the database"),
    }
}

#[post("/groups/new", data = "<newgroup>")]
pub fn newgroup_post(conn: ObservDbConn, l: AdminGuard, newgroup: Form<NewGroup>) -> Redirect {
    use crate::schema::groups::dsl::*;

    insert_into(groups).values(&newgroup.into_inner()).execute(&*conn).expect("Failed to insert group into the database");
    Redirect::to("/groups")
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

#[delete("/groups/<gid>")]
pub fn group_delete(conn: ObservDbConn, l: AdminGuard, gid: i32) -> Redirect {
    use crate::schema::groups::dsl::*;
    delete(groups.find(gid))
        .execute(&*conn)
        .expect("Failed to delete group from database");
    Redirect::to("/groups")
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
        stories: news
            .order(happened_at.asc())
            .load(&*conn)
            .expect("Failed to get news from database"),
    }
}

#[get("/news.json")]
pub fn news_json(conn: ObservDbConn, l: MaybeLoggedIn) -> Json<Vec<NewsStory>> {
    use crate::schema::news::dsl::*;
    Json(
        news.order(happened_at.asc())
            .load(&*conn)
            .expect("Failed to get news from database"),
    )
}

#[get("/news.xml")]
pub fn news_rss(conn: ObservDbConn) {
    unimplemented!()
}

#[get("/news/<nid>")]
pub fn newsstory(conn: ObservDbConn, l: MaybeLoggedIn, nid: i32) -> NewsStoryTemplate {
    use crate::schema::news::dsl::*;
    NewsStoryTemplate {
        logged_in: l.user(),
        story: news
            .find(nid)
            .first(&*conn)
            .expect("Failed to get news event from database"),
    }
}

#[get("/news/new")]
pub fn newnewsstory(conn: ObservDbConn, l: AdminGuard) -> NewNewsStoryTemplate {
    NewNewsStoryTemplate {
        logged_in: Some(l.0),
    }
}

#[post("/news/new", data = "<newnewsstory>")]
pub fn newnewsstory_post(
    conn: ObservDbConn,
    l: AdminGuard,
    newnewsstory: Form<NewNewsStory>,
) -> Redirect {
    use crate::schema::news::dsl::*;

    insert_into(news)
        .values(&newnewsstory.into_inner())
        .execute(&*conn)
        .expect("Failed to insert news story into database");

    Redirect::to("/news")
}

#[get("/news/<nid>/edit")]
pub fn editnewsstory(conn: ObservDbConn, l: AdminGuard, nid: i32) -> EditNewsStoryTemplate {
    use crate::schema::news::dsl::*;
    EditNewsStoryTemplate {
        logged_in: Some(l.0),
        story: news
            .find(nid)
            .first(&*conn)
            .expect("Failed to load news story from database"),
    }
}

#[put("/news/<nid>", data = "<editnewsstory>")]
pub fn editnewsstory_put(
    conn: ObservDbConn,
    l: AdminGuard,
    editnewsstory: Form<NewNewsStory>,
    nid: i32,
) -> Redirect {
    use crate::schema::news::dsl::*;

    update(news.find(nid))
        .set(&editnewsstory.into_inner())
        .execute(&*conn)
        .expect("Failed to update news story in the database");

    Redirect::to(format!("/news/{}", nid))
}

#[delete("/news/<nid>")]
pub fn newsstory_delete(conn: ObservDbConn, l: AdminGuard, nid: i32) -> Redirect {
    use crate::schema::news::dsl::*;
    delete(news.find(nid))
        .execute(&*conn)
        .expect("Failed to delete news story from database");
    Redirect::to("/news")
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
