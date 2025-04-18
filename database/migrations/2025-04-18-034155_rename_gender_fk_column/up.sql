-- Your SQL goes here

ALTER TABLE Game RENAME COLUMN gender_id TO game_gender_id;

ALTER TABLE Game DROP CONSTRAINT game_gender_id_fkey;

ALTER TABLE Game ADD CONSTRAINT game_game_gender_id_fkey
  FOREIGN KEY (game_gender_id) REFERENCES GameGender(id) ON DELETE RESTRICT;