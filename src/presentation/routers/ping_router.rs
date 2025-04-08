use rocket::{get, routes, serde::json::Json, Route};
use serde::Serialize;

pub fn get_routes() -> Vec<Route> {
  routes![ping]
}

#[derive(Serialize)]
struct PingResponse {
  ping: String
}

#[get("/ping")]
async fn ping() -> Json<PingResponse> {
  Json(PingResponse { ping: String::from("pong") })
}