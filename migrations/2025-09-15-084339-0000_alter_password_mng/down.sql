-- This file should undo anything in `up.sql`
ALTER TABLE password_manager
ADD COLUMN salt VARCHAR(255) DEFAULT '' NOT NULL;
