use nanoid::nanoid;
use tokio::fs;

use crate::module::{
    model::ServerResult,
    project::{dto::ProjectCreatePayload, repository},
};

pub async fn create(project_create_payload: ProjectCreatePayload) -> ServerResult<()> {
    let _ = repository::insert_one(&project_create_payload).await?;

    // 生成表结构
    let project_id = nanoid!(5);

    // 创建基本目录结构
    let path_list = ["input", "cache", "output"];
    for path in path_list {
        fs::create_dir_all(format!("project/{}/{}", project_id, path))
            .await
            .unwrap();
    }

    Ok(())
}
