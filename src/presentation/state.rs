use crate::infrastructure::database::core::context::Context;

pub struct State {
  pub context: Context
}

impl State {
  pub fn build_context() -> Context {
    Context::new()
  }

  pub fn new() -> Self {
    let context: Context = Self::build_context();

    Self { context }
  }
}