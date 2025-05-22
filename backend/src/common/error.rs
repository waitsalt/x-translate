use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // 系统错误
    ConfigError,
    SqlActionError,

    // llm
    LlmError,

    // 其他问题
    Other,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(_: config::ConfigError) -> Self {
        AppError::ConfigError
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::SqlActionError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            // 系统错误
            AppError::ConfigError => (StatusCode::INTERNAL_SERVER_ERROR, 1001, "服务配置文件错误"),
            AppError::SqlActionError => (StatusCode::INTERNAL_SERVER_ERROR, 1002, "数据库操作错误"),

            // llm
            AppError::LlmError => (StatusCode::INTERNAL_SERVER_ERROR, 1003, "LLM操作错误"),

            // 其他问题
            AppError::Other => (StatusCode::FORBIDDEN, 9000, "未知错误"),
            _ => (StatusCode::FORBIDDEN, 9000, "未知错误"),
        };
        let body = Json(json!({
            "code": code,
            "message":message,
            "data":()
        }));
        (status_code, body).into_response()
    }
}
