ALTER TABLE user_data
ADD CONSTRAINT uq_user_field_name_email UNIQUE (user_email);