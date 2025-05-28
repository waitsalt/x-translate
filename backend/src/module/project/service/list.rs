use crate::module::{
    model::ServerResult,
    project::{dto::ProjectListParam, model::Project, repository},
};

pub async fn list(project_list_param: &ProjectListParam) -> ServerResult<Vec<Project>> {
    let result = repository::select_some_by_name(project_list_param)
        .await
        .unwrap();
    Ok(result)
}
