use diesel::prelude::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::core::schema::game;
use crate::domain::models::game_gender;
use chrono::NaiveDate;

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(game_gender::GameGender))]
#[diesel(table_name = game)]
pub struct Game {
  pub id: i32,
  pub name: String,
  pub players: i32,
  pub release_date: NaiveDate,
  pub game_gender_id: i32,
}