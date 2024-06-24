/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub(crate) use self::utils::*;
pub use self::{api::AppServer, application::App};

pub(crate) mod api;
pub(crate) mod application;

pub(crate) mod utils {

    pub(crate) async fn shutdown() {
        tokio::signal::ctrl_c()
            .await
            .expect("CTRL+C: shutdown failed");

        tracing::info!("Shutdown the server...");
    }
}
