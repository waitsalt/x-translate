use serde::{Deserialize, Serialize};

use crate::common::{error::AppError, response::AppResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct SqlQueryResultListWithCount<T> {
    count: i64,
    list: Vec<T>,
}

pub type ServerResult<T> = Result<T, AppError>;
pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum PartType {
    Line,  // 行
    Token, // 词
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum BaseLanguage {
    zh_CN,
    en_US,
    ja_JP,
    ko_KR,
}
