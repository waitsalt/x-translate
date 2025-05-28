use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handler;

pub fn init() -> Router {
    Router::new()
        .route("/", get(handler::list))
        .route("/", post(handler::create))
        .route("/{project_id}", get(handler::info))
        .route("/{project_id}", put(handler::update))
        .route("/{project_id}", delete(handler::delete))
        .route("/{project_id}/start", get(handler::start))
}
