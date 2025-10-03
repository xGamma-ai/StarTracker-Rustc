-- Your SQL goes here
ALTER TABLE user_settings
DROP COLUMN IF EXISTS addtional_settings;

ALTER TABLE user_settings
ADD COLUMN addtional_settings JSONB;