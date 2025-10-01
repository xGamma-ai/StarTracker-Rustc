use crate::user_access_management::jwt::{UserToken, gen_jwt, verify_jwt};

#[test]
fn jwt_create() {
    let _gen_jwt = gen_jwt(UserToken {
        user_email: "alwintest@123.com".to_string(),
    })
    .unwrap();
}

#[test]
fn jwt_verify_both_ways() {
    let gen_jwt = gen_jwt(UserToken {
        user_email: "alwintest@123.com".to_string(),
    })
    .unwrap();
    let get_user_details = verify_jwt(&gen_jwt).unwrap();
    assert_eq!(
        String::from("alwintest@123.com"),
        get_user_details.user_details.user_email
    )
}
