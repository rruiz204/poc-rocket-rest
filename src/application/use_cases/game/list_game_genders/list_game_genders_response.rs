use serde::Serialize;

#[derive(Serialize)]
pub struct ListGameGendersResponse {
  pub id: i32,
  pub name: String,
}

impl ListGameGendersResponse {
  pub fn new(id: i32, name: String) -> Self {
    Self { id, name }
  }
}