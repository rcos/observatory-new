use std::env;

use diesel::prelude::*;
use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::State;

#[derive(StateData)]
pub struct DatabaseMiddlewareData {
    pub connection: SqliteConnection,
}

#[derive(Clone, NewMiddleware)]
pub struct DatabaseMiddleware;

impl Middleware for DatabaseMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(DatabaseMiddlewareData {
            connection: init_database(),
        });

        Box::new(chain(state).and_then(|x| future::ok(x)))
    }
}

const FALLBACK_DB_URL: &str = "db.sqlite";

fn init_database() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!(
            "DATABASE_URL env variable not set, falling back to '{}'",
            FALLBACK_DB_URL
        );
        String::from(FALLBACK_DB_URL)
    });

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to database at {}", database_url))
}
