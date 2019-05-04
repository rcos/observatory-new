//! User authentication
//!
//! Handles user singup and login as well as the crypto-related
//! tasks of authentication.
//!
//! This module has no models.
//!
//! ## Routes
//! - `/login`
//! - `/signup`

pub mod crypto;
pub mod handlers;

mod templates;
