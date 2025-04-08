use rocket::Route;

pub mod ping_router;
pub mod gender_router;

pub fn get_routes() -> Vec<Route> {
  let mut routes: Vec<Route> = Vec::new();

  routes.extend(ping_router::get_routes());

  routes
}