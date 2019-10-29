//! User authentication cryptography
//!
//! This module handles the encryption and verification of user passwords.
//! Uses the [`ring`](https://crates.io/crates/ring) library to do encryption.

use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};

const N_ITER: u32 = 100000;
const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

/// This is a simple type around String to alert
/// other developers that this is an unsafe string.
/// Do not attempt to parse or otherwise do anything with this string.
/// It is exclusively used by the cryptography systems for database
/// compatability within `HashedPassword` and the `User` struct.
pub type UnsafeBinaryString = String;

/// Structure containing the result of a cryptographically hashed password.
/// This allows us to handle them more safely and avoid using unsafe
/// strings as much as possible.
/// This also makes the resulting API more elegant and easier to use.
///
/// The methods on this struct are unsafe code
/// (currently the only in the project)
/// due to Diesel requiring that the password hash and salt to be strings.
pub struct HashedPassword {
    pass: Vec<u8>,
    salt: Vec<u8>,
}

impl HashedPassword {
    /// Returns the hashed password as an `UnsafeBinaryString`
    ///
    /// This function returns an **invalid string** of bytes.
    /// Do not attempt to parse or otherwise do anything with this string.
    pub fn pass(&self) -> UnsafeBinaryString {
        unsafe { String::from_utf8_unchecked(self.pass.clone()) }
    }

    /// Returns the password's salt as an `UnsafeBinaryString`
    ///
    /// This function returns an **invalid string** of bytes.
    /// Do not attempt to parse or otherwise do anything with this string.
    pub fn salt(&self) -> UnsafeBinaryString {
        unsafe { String::from_utf8_unchecked(self.salt.clone()) }
    }

    /// Returns the a tuple containing both the password hash and the salt,
    /// in that order. Useful for pattern matching.
    ///
    /// This function returns an **invalid string** of bytes.
    /// Do not attempt to parse or otherwise do anything with this string.
    pub fn both(&self) -> (UnsafeBinaryString, UnsafeBinaryString) {
        (self.pass(), self.salt())
    }
}

/// Encrypt a password and return a `HashedPassword` struct
/// which can be used to get the hash and salt directly.
pub fn hash_password(pass: String) -> HashedPassword {
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
        pass.as_bytes(),
        &mut out,
    );

    // Return a HashedPassword struct
    HashedPassword {
        pass: out.to_vec(),
        salt: salt.to_vec(),
    }
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
