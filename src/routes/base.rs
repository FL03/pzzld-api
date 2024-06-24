/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::ArcCtx;
use axum::{
    extract::{Json, Path},
    Extension,
};
use serde_json::Value;

pub async fn home(Extension(ctx): Extension<ArcCtx>) -> Json<Value> {
    let data = serde_json::json!({
        "message": "Hello, World!",
        "mode": ctx.settings().mode()
    });
    axum::Json(data)
}

pub async fn get_sample(id: Path<String>) -> Json<Value> {
    let data = serde_json::json!({
        "id": id.as_str(),
    });
    axum::Json(data)
}

pub async fn post_sample(id: Path<String>) -> Json<Value> {
    let data = serde_json::json!({
        "id": id.as_str(),
    });
    axum::Json(data)
}
