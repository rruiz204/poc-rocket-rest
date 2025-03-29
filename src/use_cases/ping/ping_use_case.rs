use rocket::{async_trait, serde::json::Json};

use crate::use_cases::use_case::UseCase;

use super::ping_response::PingResponse;

pub struct PingUseCase {}

#[async_trait]
impl UseCase<String, Json<PingResponse>> for PingUseCase {
  async fn execute(&self, input: String) -> Json<PingResponse> {
    Json(PingResponse { ping: input })
  }
}