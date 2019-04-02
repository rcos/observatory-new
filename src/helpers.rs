use crate::models::*;
use diesel::prelude::*;
use ring::{digest, pbkdf2};

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

pub fn gen_hash() -> String {

}

pub fn hash_password(pass: String, key: String) -> String {
    const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

    let mut out = [0u8; CRE_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        N_ITER,
        salt: &[u8],
        pass.as_bytes(),
        &mut out
    );
}

pub fn verify_password(pass: String, compare_to: String key: String) -> bool {
    const CRE_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

    let mut out = [0u8; CRE_LEN];
    pbkdf2::verify(
        &digest::SHA512,
        N_ITER,
        salt: &[u8],
        pass.as_bytes();
        compare_to.as_bytes(),
    );
}
