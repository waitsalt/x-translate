use crate::module::{
    model::ServerResult,
    project::{model::Project, repository},
};

pub async fn info(project_id: u32) -> ServerResult<Project> {
    let project = repository::select_one_by_id(project_id).await?;
    Ok(project)
}
