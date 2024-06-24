/*
    Appellation: pzzld-api <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{app::App, config::prelude::*};

pub mod config;
pub mod routes;

mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Settings::build_from_file("Puzzled.toml")?;
    println!("{config}");
    let app = App::from_config(config).with_tracing().init();
    tracing::info!("Welcome to Puzzled!");
    app.serve().await?;

    Ok(())
}
