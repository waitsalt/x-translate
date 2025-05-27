use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handler;

pub fn init() -> Router {
    Router::new()
        .route("/", get(handler::list))
        .route("/", post(handler::create))
        .route("/{interface_id}", get(handler::info))
        .route("/{interface_id}", delete(handler::delete))
        .route("/{interface_id}", put(handler::update_base_info))
        .route("/{interface_id}/test", post(handler::test))
}
