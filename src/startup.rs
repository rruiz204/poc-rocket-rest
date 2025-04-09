use rocket::{Rocket, Build};

use crate::presentation::{routers, state::State};

pub fn startup() -> Rocket<Build> {
  rocket::build()
    .manage(State::new())
    .mount("/api", routers::get_routes())
}