/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Server;
use crate::config::{Context, Settings};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct App {
    ctx: Arc<Context>,
    server: Server,
}

impl App {
    pub fn new() -> Self {
        Self::from_config(Settings::build().unwrap_or_default())
    }

    pub fn from_config(config: Settings) -> Self {
        let ctx = {
            let tmp = Context::from_config(config);
            Arc::new(tmp)
        };
        let server = Server::new(ctx.clone());
        Self { ctx, server }
    }

    pub async fn bind_server(&self) -> std::io::Result<TcpListener> {
        self.server.server_addr().bind().await
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
    // TODO: Implement init() for App
    pub fn init(self) -> Self {
        tracing::info!("Initialization successful");
        self
    }

    pub fn with_tracing(self) -> Self {
        self.ctx.init_tracing();

        self
    }

    pub async fn serve(self) -> std::io::Result<()> {
        let server = super::Server::new(self.ctx);
        server.serve().await
    }
}
