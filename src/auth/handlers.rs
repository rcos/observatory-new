use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;

use crate::guards::*;
use crate::models::NewRelationGroupUser;
use crate::models::{NewUser, User};
use crate::ObservDbConn;

use super::crypto::*;
use super::templates::*;

#[get("/signup")]
pub fn signup(l: MaybeLoggedIn) -> SignUpTemplate {
    SignUpTemplate {
        logged_in: l.user(),
    }
}

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
