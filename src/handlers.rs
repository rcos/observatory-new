use askama::Template;
use diesel::insert_into;
use diesel::prelude::*;
use gotham::helpers::http::response::create_temporary_redirect;
use gotham::state::{FromState, State};
use hyper::{Body, Response};

use super::middleware::*;
use super::models::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct Index {
    pub version: &'static str,
}

pub fn index(state: State) -> (State, Index) {
    (state, Index { version: VERSION })
}

pub fn user(state: State) -> (State, User) {
    let db = DatabaseMiddlewareData::borrow_from(&state);

    use super::schema::users::dsl::*;

    // TODO path handling
    let user = users
        .filter(id.eq(0))
        .first::<User>(&db.connection)
        .expect("Error getting user");

    (state, user)
}

#[derive(Template, Default)]
#[template(path = "signup.html")]
pub struct SignUp;

pub fn signup_get(state: State) -> (State, SignUp) {
    (state, SignUp::default())
}

pub fn signup_post(mut state: State) -> (State, Response<Body>) {
    // use super::schema::users::dsl::*;

    #[derive(Debug, StateData)]
    struct SignupPost {
        email: String,
        password: String,
    }

    let postdata = SignupPost::take_from(&mut state);
    /*
    let db = DatabaseMiddlewareData::borrow_from(&state);
    let newuser = NewUser::default();

    insert_into(users)
        .values(vec![newuser])
        .execute(&db.connection)
        .expect("Failed to insert user");
    */

    dbg!(postdata);

    let res = create_temporary_redirect(&state, "/");
    (state, res)
}
