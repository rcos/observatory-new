//! Attendance code generation and verification functions

use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::Attendable;
use crate::models::Event;
use crate::models::Meeting;

/// Verify that an attendance code is valid
///
/// Takes a reference to the database connection and the code you want
/// to verify and returns the event that the code corresponds to if it exists.
pub fn verify_code(conn: &SqliteConnection, vcode: &str) -> Option<Box<dyn Attendable>> {
    if let Some(e) = {
        use crate::schema::events::dsl::*;
        events
            .filter(code.eq(vcode.to_lowercase()))
            .first::<Event>(conn)
            .optional()
            .expect("Failed to get events from database")
    } {
        Some(Box::new(e))
    } else if let Some(m) = {
        use crate::schema::meetings::dsl::*;
        meetings
            .filter(code.eq(vcode.to_lowercase()))
            .first::<Meeting>(conn)
            .optional()
            .expect("Failed to get meetings from database")
    } {
        Some(Box::new(m))
    } else {
        None
    }
}

/// Generate a **unique** attendance code
///
/// Takes a reference to the database connection and returns a
/// **unique** attendance code that has not been used before.
pub fn attendance_code(conn: &SqliteConnection) -> String {
    let code = gen_code();
    if verify_code(conn, &code).is_some() {
        attendance_code(conn)
    } else {
        code
    }
}

/// Generate an attendance code
///
/// This function generates a random 6 digit alphanumeric string.
///
/// Generally speaking you are going to want to use `attendance_code()`
/// instead of this function, because `attendance_code()` verifies
/// that the code is unique and has not been used before.
pub fn gen_code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(6)
        .collect::<String>()
        .to_lowercase()
}
