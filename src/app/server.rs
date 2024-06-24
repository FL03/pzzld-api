/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::ServerBuilder;
use crate::config::Context;

use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    ctx: Arc<Context>,
    router: Router,
}

impl Server {
    pub fn new(ctx: Arc<Context>) -> Self {
        let router = ServerBuilder::new()
            .routes()
            .serve_file("./assets/index.html")
            .with_context(ctx.clone())
            .with_tracing()
            .build();
        Self { ctx, router }
    }

    pub fn from_context(ctx: Context) -> Self {
        let ctx = Arc::new(ctx);
        let router = ServerBuilder::new()
            .routes()
            .serve_file("./assets/index.html")
            .with_context(ctx.clone())
            .with_tracing()
            .build();
        Self { ctx, router }
    }

    pub async fn listen(&self) -> std::io::Result<TcpListener> {
        self.server_addr().bind().await
    }

    pub fn server_addr(&self) -> &crate::config::ServerAddr {
        self.ctx().settings().server_addr()
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub async fn serve(self) -> std::io::Result<()> {
        let listener = self.listen().await?;
        let router = self.router.into_make_service();

        axum::serve(listener, router)
            .with_graceful_shutdown(super::shutdown())
            .await
    }

    pub fn with_context(self, ctx: Context) -> Self {
        Self {
            ctx: Arc::new(ctx),
            ..self
        }
    }

    pub fn with_router(self, router: Router) -> Self {
        Self { router, ..self }
    }
}

mod builder {
    use crate::config::Context;
    use crate::routes::api_router;
    use axum::{
        extract::Request,
        response::IntoResponse,
        routing::{MethodRouter, Route},
        Router,
    };
    use core::convert::Infallible;
    use std::sync::Arc;
    use tower::{Layer, Service};
    use tower_http::{classify::StatusInRangeAsFailures, services, trace::TraceLayer};

    pub struct ServerBuilder<S = ()> {
        router: Router<S>,
    }

    impl<S> ServerBuilder<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        pub fn new() -> Self {
            Self {
                router: Router::<S>::new(),
            }
        }

        pub fn from_router(router: Router<S>) -> Self {
            Self { router }
        }

        pub fn build(self) -> Router<S> {
            self.router
        }

        pub fn routes(self) -> Self {
            self.nest("/api", api_router())
        }

        pub fn serve_dir(self, path: &str, workdir: &str) -> Self {
            self.route_service(path, services::ServeDir::new(workdir))
        }

        pub fn serve_file(self, target: &str) -> Self {
            self.route_service("/", services::ServeFile::new(target))
        }

        pub fn with_context(self, ctx: Arc<Context>) -> Self {
            self.layer(axum::Extension(ctx))
        }

        pub fn with_tracing(self) -> Self {
            self.layer(TraceLayer::new(
                StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
            ))
        }
    }

    impl<S> ServerBuilder<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        pub fn layer<L>(self, layer: L) -> Self
        where
            L: Clone + Layer<Route> + Send + 'static,
            L::Service: Clone + Service<Request> + Send + 'static,
            <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
            <L::Service as Service<Request>>::Future: Send + 'static,
            <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        {
            let router = self.router.layer(layer);
            Self { router }
        }
        pub fn nest(self, path: &str, router: Router<S>) -> Self {
            let router = self.router.nest(path, router);
            Self { router }
        }

        pub fn route(self, path: &str, method: MethodRouter<S>) -> Self {
            let router = self.router.route(path, method);
            Self { router }
        }

        pub fn route_service<T>(self, path: &str, svc: T) -> Self
        where
            T: Clone + Send + Service<Request, Error = Infallible> + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            let router = self.router.route_service(path, svc);
            Self { router }
        }
    }
}
