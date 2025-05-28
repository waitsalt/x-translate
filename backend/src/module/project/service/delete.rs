use tokio::fs;

use crate::{
    common::error::AppError,
    module::{model::ServerResult, project::repository},
};

pub async fn delete(project_id: u32) -> ServerResult<()> {
    // 删除表数据
    let _ = repository::delete_one_by_id(project_id).await?;

    // 删除对应文件夹
    let project_path = format!("project/{}", project_id);
    let result = fs::remove_dir_all(project_path).await;

    match result {
        Ok(()) => Ok(()),
        Err(_) => Err(AppError::Other),
    }
}
