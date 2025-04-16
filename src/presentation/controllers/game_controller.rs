use crate::application::use_cases::game::list_games::{
  list_games_query::ListGamesQuery,
  list_games_use_case::ListGamesUseCase,
  list_games_response::ListGamesResponse,
};

pub struct GameController {
  list_games_use_case: ListGamesUseCase
}

impl GameController {
  pub fn new(list_games_use_case: ListGamesUseCase) -> Self {
    Self {
      list_games_use_case
    }
  }

  pub async fn list_games(&self) -> Vec<ListGamesResponse> {
    let query = ListGamesQuery { page: 1, limit: 10 };
    self.list_games_use_case.execute(query).await
  }
}