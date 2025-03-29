use rocket::{get, routes, serde::json::Json, Route};

use crate::use_cases::ping::ping_response::PingResponse;

pub fn get_routes() -> Vec<Route> {
  routes![ping]
}

#[get("/ping")]
async fn ping() -> Json<PingResponse> {
  Json(PingResponse {
    ping: String::from("pong")
  })
}