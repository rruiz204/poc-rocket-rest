use std::sync::Arc;
use rocket::{Rocket, Build};
use crate::presentation::controllers;

use crate::presentation::containers::game_container::GameContainer;

pub struct Startup {}

impl Startup {
  pub fn build() -> Rocket<Build> {
    let game_container = Arc::new(GameContainer::new());

    rocket::build()
      .manage(game_container)
      .mount("/api", controllers::get_routes())
  }
}