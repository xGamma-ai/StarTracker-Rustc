use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn login_password_hasher(password: &String) -> (Vec<u8>, String) {
    let salt = SaltString::generate(&mut OsRng);
    let mut out: Vec<u8> = vec![0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut out)
        .unwrap();
    return (out, salt.to_string());
}
