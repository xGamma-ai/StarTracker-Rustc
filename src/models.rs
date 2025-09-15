use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserData {
    pub id: i32,
    pub user_name: String,
    pub user_email: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_data)]
pub struct WriteNewUser<'a> {
    pub user_name: &'a str,
    pub user_email: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::password_manager)]
pub struct WriteNewUserPassword {
    pub user_id: i32,
    pub password_hash: String,
    pub salt: String,
}
