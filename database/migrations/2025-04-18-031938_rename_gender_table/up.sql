-- Your SQL goes here

ALTER TABLE Gender RENAME TO GameGender;

ALTER TABLE Game DROP CONSTRAINT game_gender_id_fkey;

ALTER TABLE Game ADD CONSTRAINT game_gender_id_fkey
  FOREIGN KEY (gender_id) REFERENCES GameGender(id) ON DELETE RESTRICT;