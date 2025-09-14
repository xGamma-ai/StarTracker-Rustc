// @generated automatically by Diesel CLI.

diesel::table! {
    password_manager (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_data (id) {
        id -> Int4,
        user_name -> Varchar,
        user_google_id -> Text,
        user_email -> Text,
        user_created_at -> Timestamp,
        user_updated_at -> Timestamp,
    }
}

diesel::joinable!(password_manager -> user_data (user_id));

diesel::allow_tables_to_appear_in_same_query!(password_manager, user_data,);
