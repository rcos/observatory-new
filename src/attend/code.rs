use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::calendar::Event;
use crate::groups::Meeting;
use crate::models::Attendable;

pub fn verify_code(conn: &SqliteConnection, vcode: &String) -> Option<Box<dyn Attendable>> {
    if let Some(e) = {
        use crate::schema::events::dsl::*;
        events
            .filter(code.eq(vcode))
            .first::<Event>(conn)
            .optional()
            .expect("Failed to get events from database")
    } {
        Some(Box::new(e))
    } else {
        if let Some(m) = {
            use crate::schema::meetings::dsl::*;
            meetings
                .filter(code.eq(vcode))
                .first::<Meeting>(conn)
                .optional()
                .expect("Failed to get meetings from database")
        } {
            Some(Box::new(m))
        } else {
            None
        }
    }
}

pub fn attendance_code(conn: &SqliteConnection) -> String {
    let code = gen_code();
    if verify_code(conn, &code).is_some() {
        attendance_code(conn)
    } else {
        code
    }
}

pub fn gen_code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .collect::<String>()
        .to_lowercase()
}
