use tokio::fs;

use crate::module::{
    model::ServerResult,
    project::{dto::ProjectCreatePayload, model::Project, repository},
};

pub async fn create(project_create_payload: ProjectCreatePayload) -> ServerResult<Project> {
    // 创建表数据
    let _ = repository::insert_one(&project_create_payload).await?;

    // 获取表数据
    let project = repository::select_one_by_name(&project_create_payload.project_name).await?;

    // 创建基本目录结构
    let path_list = ["input", "cache", "output"];
    for path in path_list {
        fs::create_dir_all(format!("project/{}/{}", project.project_id, path))
            .await
            .unwrap();
    }

    Ok(project)
}
