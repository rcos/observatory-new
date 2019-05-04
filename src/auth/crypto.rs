//! User authentication cryptography
//!
//! This module handles the encryption and verification of user passwords.
//! Uses the [`ring`](https://crates.io/crates/ring) library to do encryption.

use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};

const N_ITER: u32 = 100000;
const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

/// Generate password salt
///
/// This function generates **an invalid string** of bytes that is a salt
/// to be used when hashing a password.
/// Do not attempt to parse or otherwise do anything with this string.
pub fn gen_salt() -> String {
    let rng = SystemRandom::new();
    let mut salt = [0u8; CRE_LEN];
    rng.fill(&mut salt).unwrap();
    unsafe { String::from_utf8_unchecked(salt.to_vec()) }
}

/// Encrypt a password using a salt
///
/// Using a salt generate by `gen_salt` this function encrypts a password.
/// This function **returns an invalid string** of bytes.
/// Do not attempt to parse or otherwise do anything with this string.
pub fn hash_password(pass: String, salt: &String) -> String {
    let mut out = [0u8; CRE_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        N_ITER,
        salt.as_bytes(),
        pass.as_bytes(),
        &mut out,
    );
    unsafe { String::from_utf8_unchecked(out.to_vec()) }
}

/// Verify that a password is correct
///
/// Takes a password and the hash and salt of an encrypted password and
/// verifies that the password is correct.
///
/// You should never directly compare two hashed passwords.
pub fn verify_password(pass: String, compare_to: String, salt: &String) -> bool {
    pbkdf2::verify(
        &digest::SHA512,
        N_ITER,
        salt.as_bytes(),
        pass.as_bytes(),
        compare_to.as_bytes(),
    )
    .is_ok()
}
