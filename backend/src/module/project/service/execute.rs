use std::sync::Arc;

use tokio::{fs, sync::Semaphore};

use crate::module::{
    model::ServerResult,
    {project::model::Project, task::model::Task, worker::model::Worker},
};

pub async fn execute(project: Project) -> ServerResult<()> {
    // 获取任务列表
    let task_list_path = format!("project/{}/cache/task_list.txt", project.project_id);
    // let task_list_result = fs::read_to_string(task_list_path).await;
    let task_list: Vec<Task> = match fs::read_to_string(task_list_path).await {
        Ok(task_list_string) => serde_json::from_str(&task_list_string).unwrap(),
        Err(_) => Vec::new(),
    };

    let semaphore = Arc::new(Semaphore::new(project.worker_max_number as usize));
    let mut worker_list = Vec::new();
    for i in 0..project.worker_max_number {
        let semaphore_clone = semaphore.clone();
        let worker = tokio::spawn(async move {
            let _permit = semaphore_clone.acquire().await.unwrap();
            // TODO: Implement worker logic
            // let worker = Worker::create(i, project.project_id, task_index, task_list);
            // worker.execute(agent).await;
        });
        worker_list.push(worker);
    }
    Ok(())
}
