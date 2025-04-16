use list_games::list_games_use_case::ListGamesUseCase;

pub mod list_games;
pub mod list_game_genders;

pub struct GameUseCases {
  pub list_games_use_cases: ListGamesUseCase,
}