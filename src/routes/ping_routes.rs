use rocket::{get, routes, serde::json::Json, Route};

use crate::use_cases::{ping::{
  ping_response::PingResponse,
  ping_use_case::PingUseCase},
  use_case::UseCase};

pub fn get_routes() -> Vec<Route> {
  routes![ping]
}

#[get("/ping")]
async fn ping() -> Json<PingResponse> {
  let use_case: PingUseCase = PingUseCase {};
  use_case.execute(String::from("pong")).await
}