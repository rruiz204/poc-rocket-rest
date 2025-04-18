-- This file should undo anything in `up.sql`

ALTER TABLE Game DROP CONSTRAINT game_game_gender_id_fkey;

ALTER TABLE Game RENAME COLUMN game_gender_id TO gender_id;

ALTER TABLE Game ADD CONSTRAINT game_gender_id_fkey
  FOREIGN KEY (gender_id) REFERENCES GameGender(id) ON DELETE RESTRICT;