-- This file should undo anything in `up.sql`

ALTER TABLE Game DROP CONSTRAINT game_gender_id_fkey;

ALTER TABLE GameGender RENAME TO Gender;

ALTER TABLE Game ADD CONSTRAINT game_gender_id_fkey
  FOREIGN KEY (gender_id) REFERENCES Gender(id) ON DELETE RESTRICT;