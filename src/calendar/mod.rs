//! Built-in calendar
//!
//! After many years of using horrible external calendar and spreadsheets
//! we now have proper built-in calendar.
//!
//! ## Routes
//! - `/calendar`
//! - `/calendar.json`
//! - `/calendar/new`
//! - `/calendar/<eid>`
//! - `/calendar/<eid>/edit`

pub mod handlers;
pub mod models;

mod templates;
