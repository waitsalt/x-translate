use crate::module::{
    model::ServerResult,
    project::{dto::ProjectUpdatePayload, model::Project, repository},
};

pub async fn update(
    project_id: u32,
    project_update_payload: &ProjectUpdatePayload,
) -> ServerResult<Project> {
    let _ = repository::update_one_by_id(project_id, project_update_payload).await?;
    let project = repository::select_one_by_id(project_id).await?;
    Ok(project)
}
