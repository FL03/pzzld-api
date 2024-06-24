/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::{Context, Settings};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct App {
    ctx: Arc<Context>,
}

impl App {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx: Arc::new(ctx) }
    }

    pub fn from_config(config: Settings) -> Self {
        let ctx = Context::from_config(config);

        Self { ctx: Arc::new(ctx) }
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
    // TODO: Implement init() for App
    pub fn init(self) -> Self {
        self
    }

    pub fn with_tracing(self) -> Self {
        self.ctx.init_tracing();

        self
    }

    pub async fn serve(self) -> std::io::Result<()> {
        use super::AppServer;
        let server = AppServer::new(self.ctx);
        server.serve().await
    }
}

impl App {
    pub async fn bind_server(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(self.ctx.settings().server_addr().addr()).await
    }
}
/*
 ************* Private *************
*/
impl App {
    // fn router(&self) -> axum::Router {
    //     use super::AppServer;

    //     let server = AppServer::new(self.ctx.clone());
    //     server.build().
    // }
}
