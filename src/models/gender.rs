use diesel::prelude::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::gender;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = gender)]
pub struct Gender {
  pub id: i32,
  pub name: String,
}