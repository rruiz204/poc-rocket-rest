use rocket::{Rocket, Build};

use crate::presentation::routers;

pub struct Startup {}

impl Startup {
  pub fn build() -> Rocket<Build> {
    rocket::build()
      .mount("/api", routers::get_routes())
  }
}