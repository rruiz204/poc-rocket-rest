use rocket::Rocket;

use crate::presentation::routers;

pub fn startup() -> Rocket<rocket::Build> {
  rocket::build()
    .mount("/api", routers::get_routes())
}