use std::sync::Arc;
use rocket::{get, routes, serde::json::Json, Route, State};
use crate::presentation::containers::game_container::GameContainer;

use crate::application::use_cases::game::list_games::{
  list_games_query::ListGamesQuery,
  list_games_response::ListGamesResponse,
};

use crate::application::use_cases::game::list_game_genders::{
  list_game_genders_query::ListGameGendersQuery,
  list_game_genders_response::ListGameGendersResponse,
};

pub struct GameController {}

impl GameController {
  pub fn get_routes() -> Vec<Route> {
    routes![
      list_games,
      list_game_genders,
    ]
  }
}

#[get("/game")]
async fn list_games(container: &State<Arc<GameContainer>>) -> Json<Vec<ListGamesResponse>> {
  let query = ListGamesQuery { page: 1, limit: 10 };
  let games = container.list_games_use_case.execute(query).await;

  Json(games)
}

#[get("/game/gender")]
async fn list_game_genders(container: &State<Arc<GameContainer>>) -> Json<Vec<ListGameGendersResponse>> {
  let query = ListGameGendersQuery { page: 1, limit: 10 };
  let genders = container.list_game_genders_use_case.execute(query).await;

  Json(genders)
}