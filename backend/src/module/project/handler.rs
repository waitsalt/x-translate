use axum::{Json, extract::Path};

use crate::{common::response::AppResponse, module::model::AppResult};

use super::{
    dto::{ProjectCreatePayload, ProjectListParam, ProjectUpdatePayload},
    model::Project,
    service,
};

pub async fn create(
    Json(project_create_payload): Json<ProjectCreatePayload>,
) -> AppResult<Project> {
    let project = service::create(project_create_payload).await?;
    Ok(AppResponse::success(Some(project)))
}

pub async fn delete(Path(project_id): Path<u32>) -> AppResult<()> {
    let _ = service::delete(project_id).await?;
    Ok(AppResponse::success(None))
}

pub async fn list(Json(project_list_param): Json<ProjectListParam>) -> AppResult<Vec<Project>> {
    let result = service::list(&project_list_param).await?;
    Ok(AppResponse::success(Some(result)))
}

pub async fn info(Path(project_id): Path<u32>) -> AppResult<Project> {
    let project = service::info(project_id).await?;
    Ok(AppResponse::success(Some(project)))
}

pub async fn update(
    Path(project_id): Path<u32>,
    Json(project_update_payload): Json<ProjectUpdatePayload>,
) -> AppResult<Project> {
    let project = service::update(project_id, &project_update_payload).await?;
    Ok(AppResponse::success(Some(project)))
}

pub async fn start(Path(project_id): Path<u32>) -> AppResult<()> {
    let _ = service::start(project_id).await?;
    Ok(AppResponse::success_with_message(
        "项目启动成功".into(),
        None,
    ))
}
