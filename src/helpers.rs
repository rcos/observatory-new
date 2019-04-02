use crate::models::*;
use diesel::prelude::*;
use ring::{digest, pbkdf2, rand};
use ring::rand::SecureRandom;

pub fn filter_users(conn: &SqliteConnection, term: Option<String>) -> Vec<User> {
    use crate::schema::users::dsl::*;

    if let Some(term) = term {
        let sterm = format!("%{}%", term);
        let filter = real_name
            .like(&sterm)
            .or(email.like(&sterm))
            .or(handle.like(&sterm));
        users.filter(filter).load(conn)
    } else {
        users.load(conn)
    }
    .expect("Failed to get users")
}

const N_ITER: u32 = 100000;
const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

pub fn gen_salt() -> String {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CRE_LEN];
    rng.fill(&mut salt).unwrap();
    unsafe {
        String::from_utf8_unchecked(salt.to_vec())
    }
}

pub fn hash_password(pass: String, salt: &String) -> String {
    let mut out = [0u8; CRE_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        N_ITER,
        salt.as_bytes(),
        pass.as_bytes(),
        &mut out
    );
    unsafe {
        String::from_utf8_unchecked(out.to_vec())
    }
}

pub fn verify_password(pass: String, compare_to: String, salt: &String) -> bool {
    pbkdf2::verify(
        &digest::SHA512,
        N_ITER,
        salt.as_bytes(),
        pass.as_bytes(),
        compare_to.as_bytes(),
    ).is_ok()
}
