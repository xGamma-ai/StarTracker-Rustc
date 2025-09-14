-- Your SQL goes here
CREATE TABLE user_data (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR NOT NULL,
    user_google_id TEXT NOT NULL,
    user_email TEXT NOT NULL,
    user_created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)