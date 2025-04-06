use startup::startup;

pub mod routes;
pub mod schema;
pub mod startup;
pub mod use_cases;
pub mod controllers;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = startup();
    rocket.launch().await?;
    Ok(())
}