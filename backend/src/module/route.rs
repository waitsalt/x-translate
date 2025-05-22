use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    // 设置请求允许
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 加载路由
    Router::new()
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
