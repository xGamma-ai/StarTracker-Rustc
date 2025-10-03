// @generated automatically by Diesel CLI.

diesel::table! {
    password_manager (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        salt -> Text,
    }
}

diesel::table! {
    user_data (id) {
        id -> Int4,
        user_name -> Varchar,
        user_email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        enable_online_mode -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        addtional_settings -> Nullable<Jsonb>,
    }
}

diesel::joinable!(password_manager -> user_data (user_id));
diesel::joinable!(user_settings -> user_data (user_id));

diesel::allow_tables_to_appear_in_same_query!(password_manager, user_data, user_settings,);
