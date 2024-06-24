/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod base;
pub mod serve;

use axum::{
    routing::{get, post},
    Router,
};

pub(crate) fn api_router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", get(base::home))
        .route("/sample/:id", get(base::get_sample))
        .route("/sample/:id", post(base::post_sample))
}
