use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

use crate::module::model::ServerResult;

#[async_recursion]
pub async fn has_file_in_dir(path: &Path) -> ServerResult<bool> {
    // 检查是否为文件夹
    if !path.is_dir() {
        return Ok(false);
    }

    // 获取文件夹里的实体列表
    let mut entry_list = fs::read_dir(path).await.unwrap();

    // 判断实体是否为文件 不是则判断是否为文件夹 是文件夹则递归调用
    while let Some(entry) = entry_list.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_file() {
            return Ok(true);
        } else if path.is_dir() {
            return has_file_in_dir(&path).await;
        }
    }
    Ok(false)
}
