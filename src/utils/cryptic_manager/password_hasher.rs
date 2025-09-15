use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};
use subtle::ConstantTimeEq;

use crate::{establish_connection, models::UserPasswordDetails};

pub fn login_password_hasher(password: &String) -> (Vec<u8>, String) {
    let salt = SaltString::generate(&mut OsRng);
    let mut out: Vec<u8> = vec![0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut out)
        .unwrap();
    return (out, salt.to_string());
}

pub fn verify_pwd_state(stored_hash: &Vec<u8>, stored_salt: &String, password: &String) -> bool {
    let mut incoming_pwd_raw = vec![0u8; stored_hash.len()];
    Argon2::default()
        .hash_password_into(
            password.as_bytes(),
            stored_salt.as_bytes(),
            &mut incoming_pwd_raw,
        )
        .expect("Failed to create verify_pwd_hashing to compare.");
    return stored_hash.ct_eq(&incoming_pwd_raw).into();
}
