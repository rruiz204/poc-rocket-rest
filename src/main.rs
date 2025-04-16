use startup::Startup;

pub mod startup;

pub mod domain;
pub mod application;
pub mod presentation;
pub mod infrastructure;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = Startup::build();
    rocket.launch().await?;
    Ok(())
}