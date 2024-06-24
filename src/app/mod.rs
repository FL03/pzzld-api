/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{application::App, server::Server, utils::*};

pub(crate) mod application;
pub(crate) mod server;

pub(crate) mod utils {

    pub async fn shutdown() {
        tokio::signal::ctrl_c()
            .await
            .expect("CTRL+C: shutdown failed");

        tracing::info!("Shutdown the server...");
    }
}
