use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
  pub ping: String
}