use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

use crate::common::config::CONFIG;

use super::{interface, project, task, term};

pub fn init() -> Router {
    // 设置请求允许
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 设置各模块路由
    let interface_router = interface::router::init();
    let project_router = project::router::init();
    let task_router = task::router::init();
    let term_router = term::router::init();

    // 设置模块总路由
    let module_router = Router::new()
        .nest("/interface", interface_router)
        .nest("/project", project_router)
        .nest("/task", task_router)
        .nest("/term", term_router);

    // 从配置文件中获取路由基本路经
    let base_path = CONFIG.router.base_path.clone();

    // 返回最终路由
    Router::new()
        .nest(&base_path, module_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
