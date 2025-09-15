use StarTracker_rustc::utils::password_hasher::login_password_hasher;

#[test]
fn test_login_password_hasher() {
    let password = format!("lwin_the_humble");
    let hashed_password = login_password_hasher(password.clone());
    assert_eq!(password, hashed_password);
}
