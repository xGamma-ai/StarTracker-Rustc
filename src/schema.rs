// @generated automatically by Diesel CLI.

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
