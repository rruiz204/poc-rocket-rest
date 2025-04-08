use startup::startup;

pub mod startup;

pub mod domain;
pub mod presentation;
pub mod infrastructure;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = startup();
    rocket.launch().await?;
    Ok(())
}