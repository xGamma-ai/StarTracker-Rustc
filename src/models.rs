use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserData {
    pub id: i32,
    pub user_name: String,
    pub user_email: String,
}
