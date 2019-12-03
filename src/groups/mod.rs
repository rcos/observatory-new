//! User groups
//!
//! Users are parts of groups, either small or large and should
//! be attending the meetings of those groups.
//!
//! ## Routes
//! - `/groups`
//! - `/groups.json`
//! - `/groups/new`
//! - `/groups/<gid>`
//! - `/groups/<gid>/add`
//! - `/groups/<gid>/remove/<uid>`
//! - `/groups/<gid>/edit`
//! - `/groups/<gid>/meetings
//! - `/groups/<gid>/meetings.json
//! - `/groups/<gid>/meetings/new
//! - '/groups/<gid>/meetings/<mid>

pub mod handlers;
pub mod models;

mod templates;
