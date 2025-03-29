use rocket::Route;

mod ping_routes;

pub fn get_routes() -> Vec<Route> {
  let mut routes: Vec<Route> = Vec::new();
  routes.extend(ping_routes::get_routes());

  routes
}