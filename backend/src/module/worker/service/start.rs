use crate::module::{
    self, interface::model::Interface, model::ServerResult, project::model::Project,
    task::model::Task, worker::model::Worker,
};

pub async fn start(project: &Project, interface: &Interface, worker: &Worker) -> ServerResult<()> {
    loop {
        let task_opt: Option<Task>;
        let task_index: u32;

        {
            let task_list_guard = worker.task_list.lock().await;
            let mut task_index_guard = worker.task_index.lock().await;

            task_index = *task_index_guard;
            if (task_index as usize) < task_list_guard.len() {
                task_opt = Some(task_list_guard[task_index as usize].clone());
                *task_index_guard += 1;
            } else {
                task_opt = None;
            }
        }

        let task = if let Some(task) = task_opt {
            task
        } else {
            break;
        };
        module::task::service::start(&project, &interface, &task)
            .await
            .unwrap();
    }
    Ok(())
}
