use rocket::Route;

pub mod game_controller;

pub fn get_routes() -> Vec<Route> {
  let mut routes: Vec<Route> = Vec::new();

  routes.extend(game_controller::GameController::get_routes());

  routes
}