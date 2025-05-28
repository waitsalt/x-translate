use crate::module::{
    model::ServerResult,
    task::{dto::TaskCreatePayload, model::Task, repository},
};

pub async fn create(
    project_id: u32,
    task_create_payload_list: &[TaskCreatePayload],
) -> ServerResult<Vec<Task>> {
    let _ = repository::insert_some(project_id, task_create_payload_list).await?;
    let task_list = repository::select_some_by_project_id(project_id)
        .await
        .unwrap();
    Ok(task_list)
}
