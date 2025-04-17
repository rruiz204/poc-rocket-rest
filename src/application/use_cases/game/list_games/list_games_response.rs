use serde::Serialize;

#[derive(Serialize)]
pub struct ListGamesResponse {
  pub id: i32,
  pub name: String,
}

impl ListGamesResponse {
  pub fn new(id: i32, name: String) -> Self {
    Self { id, name }
  }
}