/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Context;
use crate::routes::base;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tower_http::{classify::StatusInRangeAsFailures, services::ServeFile, trace::TraceLayer};

pub(crate) fn _api<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", get(base::home))
        .route("/sample/:id", get(base::get_sample))
        .route("/sample/:id", post(base::post_sample))
}


pub struct AppServer {
    ctx: Arc<Context>,
    router: Router,
}

impl AppServer {
    pub fn new(ctx: Arc<Context>) -> Self {
        let router = RouterBuilder::new()
            .routes()
            .serve_file("./assets/index.html")
            .with_context(ctx.clone())
            .with_tracing()
            .build();
        Self { ctx, router }
    }

    pub fn from_context(ctx: Context) -> Self {
        Self::new(Arc::new(ctx))
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

    pub async fn serve(self) -> pzzld::IoResult<()> {
        let listener = self.server_addr().bind().await?;
        // let router = _builder(self.router).layer(Extension(self.ctx));

        axum::serve(listener, self.router.into_make_service())
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

pub struct RouterBuilder<S = ()> {
    router: Router<S>,
}


impl<S> RouterBuilder<S> where S: Clone + Send + Sync + 'static {
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
        self.nest("/api", _api())
    }

    pub fn serve_dir(self, path: &str, workdir: &str) -> Self {
        self.route_service(path, tower_http::services::ServeDir::new(workdir))
    }

    pub fn serve_file(self, target: &str) -> Self {
        self.route_service("/", ServeFile::new(target))
    }

    pub fn with_context(self, ctx: Arc<Context>) -> Self {
        self.layer(Extension(ctx))
    }

    pub fn with_tracing(self) -> Self {
        self.layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
    }

   
}

mod impl_builder {
    use super::*;

    use axum::{extract::Request, response::IntoResponse, routing::Route};
    use core::convert::Infallible;
    use tower::{Layer, Service};

    impl<S> RouterBuilder<S> where S: Clone + Send + Sync + 'static {
        pub fn layer<L>(self, layer: L) -> Self 
        where
            L: Clone + Layer<Route> + Send + 'static,
            L::Service:Clone +  Service<Request> + Send + 'static,
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

        pub fn route(self, path: &str, method: axum::routing::MethodRouter<S>) -> Self {
            let router = self.router.route(path, method);
            Self { router }
        }

        pub fn route_service<T>(self, path: &str, svc: T) -> Self     where
            T: Clone + Send + Service<Request, Error = Infallible> + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            let router = self.router.route_service(path, svc);
            Self { router }
        }
    }

}
