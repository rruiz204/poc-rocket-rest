use diesel::prelude::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::core::schema::gamegender;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = gamegender)]
pub struct GameGender {
  pub id: i32,
  pub name: String,
}