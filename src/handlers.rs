use askama::Template;
use diesel::prelude::*;
use gotham::state::{FromState, State};

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
    let data = DatabaseMiddlewareData::borrow_from(&state);

    use super::schema::users::dsl::*;

    // TODO path handling
    let user = users
        .filter(id.eq(0))
        .first::<User>(&data.database)
        .expect("Error getting user");

    (state, user)
}
