use rocket::{get, routes, Route};

pub fn get_routes() -> Vec<Route> {
  routes![
    list_games,
    list_game_genders,
  ]
}

#[get("/game")]
async fn list_games() {

}

#[get("/game/gender")]
async fn list_game_genders() {

}