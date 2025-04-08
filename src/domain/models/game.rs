use diesel::prelude::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::core::schema::game;
use crate::domain::models::gender;
use chrono::NaiveDate;

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(gender::Gender))]
#[diesel(table_name = game)]
pub struct Game {
  pub id: i32,
  pub name: String,
  pub players: i32,
  pub release_date: NaiveDate,
  pub gender_id: i32,
}