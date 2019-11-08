//! User authentication cryptography
//!
//! This module handles the encryption and verification of user passwords.
//! Uses the [`ring`](https://crates.io/crates/ring) library to do encryption.

use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};
use rocket::http::RawStr;
use rocket::request::FromFormValue;

const N_ITER: u32 = 100_000;
const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

/// A newtype struct wrapping around a `String`
/// that is used by a number of cryptography functions
/// in order to be compatible with SQLite.
/// The type itself exists so that it is not possible to accidentally
/// use it as if it were a string, providing far more safety.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, DieselNewType)]
pub struct UnsafeBinaryString(String);

impl UnsafeBinaryString {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for UnsafeBinaryString {
    fn default() -> Self {
        Self(String::new())
    }
}

impl<T> From<T> for UnsafeBinaryString
where
    T: AsRef<[u8]>,
{
    fn from(f: T) -> Self {
        unsafe { Self(String::from_utf8_unchecked(f.as_ref().to_vec())) }
    }
}

impl<'v> FromFormValue<'v> for UnsafeBinaryString {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        Ok(Self(form_value.to_string()))
    }
}

impl AsRef<str> for UnsafeBinaryString {
    fn as_ref(&self) -> &str {
        &*self.0
    }
}

/// Encrypt a password and return a `HashedPassword` struct
/// which can be used to get the hash and salt directly.
pub fn hash_password<T: AsRef<str>>(pass: T) -> (UnsafeBinaryString, UnsafeBinaryString) {
    // Generate the password salt
    let rng = SystemRandom::new();
    let mut salt = [0u8; CRE_LEN];
    rng.fill(&mut salt).unwrap();

    // Derive the password hash
    let mut out = [0u8; CRE_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        N_ITER,
        &salt as &[u8],
        pass.as_ref().as_bytes(),
        &mut out,
    );

    // Return a HashedPassword struct
    (
        UnsafeBinaryString::from(out),
        UnsafeBinaryString::from(salt),
    )
}

/// Verify that a password is correct
///
/// Takes a password and the hash and salt of an encrypted password and
/// verifies that the password is correct.
///
/// You should never directly compare two hashed passwords.
pub fn verify_password(
    pass: String,
    compare_to: UnsafeBinaryString,
    salt: UnsafeBinaryString,
) -> bool {
    pbkdf2::verify(
        &digest::SHA512,
        N_ITER,
        salt.as_bytes(),
        pass.as_bytes(),
        compare_to.as_bytes(),
    )
    .is_ok()
}
