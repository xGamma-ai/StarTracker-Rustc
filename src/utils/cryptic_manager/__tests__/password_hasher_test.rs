use argon2::Argon2;
use subtle::ConstantTimeEq;

#[test]
fn test_login_password_hasher_equal() {
    let password = format!("lwin_the_humble");
    let hashed_password = crate::utils::login_password_hasher(&password);
    let (u8_hash, salt) = hashed_password;

    let mut candidate = vec![0u8; u8_hash.len()];
    Argon2::default()
        .hash_password_into(
            password.as_bytes(),
            salt.as_str().as_bytes(),
            &mut candidate,
        )
        .unwrap();
    assert_eq!(u8_hash.ct_eq(&candidate).unwrap_u8(), 1);
    //assert_eq!(, true);
}

#[test]
fn test_login_password_hasher_not_equal() {
    let password = format!("lwin_the_humble");
    let password_test: String = format!("win_the_humble");
    let hashed_password = crate::utils::login_password_hasher(&password);
    let (u8_hash, salt) = hashed_password;

    let mut candidate = vec![0u8; u8_hash.len()];
    Argon2::default()
        .hash_password_into(
            password_test.as_bytes(),
            salt.as_str().as_bytes(),
            &mut candidate,
        )
        .unwrap();
    assert_eq!(u8_hash.ct_eq(&candidate).unwrap_u8(), 0);
    //assert_eq!(, true);
}
