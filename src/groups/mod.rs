//! User groups
//!
//! Users are parts of groups, either small or large and should
//! be attending the meetings of those groups.
//!
//! ## Routes
//! - `/groups`
//! - `/groups/new`
//! - `/groups/<gid>`
//! - `/groups/<gid>/add`
//! - `/groups/<gid>/remove/<uid>`
//! - `/groups/<gid>/edit`

pub mod handlers;
pub mod models;

mod templates;
