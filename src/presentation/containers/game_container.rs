use std::sync::Arc;

use crate::application::use_cases::game::{
  list_games::list_games_use_case::ListGamesUseCase,
  list_game_genders::list_game_genders_use_case::ListGameGendersUseCase,
};

pub struct GameContainer {
  pub list_games_use_case: Arc<ListGamesUseCase>,
  pub list_game_genders_use_case: Arc<ListGameGendersUseCase>,
}

impl GameContainer {
  pub fn new() -> Self {
    let list_games_use_case = Arc::new(ListGamesUseCase::new());
    let list_game_genders_use_case = Arc::new(ListGameGendersUseCase::new());

    Self {
      list_games_use_case,
      list_game_genders_use_case,
    }
  }
}