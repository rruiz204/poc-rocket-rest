use super::{
  list_games_query::ListGamesQuery,
  list_games_response::ListGamesResponse
};

pub struct ListGamesUseCase {}

impl ListGamesUseCase {
  pub fn new() -> Self {
    Self { }
  }

  pub async fn execute(&self, query: ListGamesQuery) -> Vec<ListGamesResponse> {
    println!("page: {}, limit: {}", query.page, query.limit);

    let games: Vec<ListGamesResponse> = vec![
      ListGamesResponse::new(1, "Game #1".to_string()),
      ListGamesResponse::new(2, "Game #2".to_string()),
      ListGamesResponse::new(3, "Game #3".to_string()),
    ];

    games
  }
}