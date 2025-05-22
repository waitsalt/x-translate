use serde::{Deserialize, Serialize};

use crate::common::{error::AppError, response::AppResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct SqlQueryResultListWithCount<T> {
    count: i64,
    list: Vec<T>,
}

pub type ServerResult<T> = Result<T, AppError>;
pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;
