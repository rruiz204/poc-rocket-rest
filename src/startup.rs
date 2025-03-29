use rocket::Rocket;

use crate::routes;

pub fn startup() -> Rocket<rocket::Build> {
  rocket::build()
    .mount("/api", routes::get_routes())
}