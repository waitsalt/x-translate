use std::sync::Arc;

use tokio::sync::Mutex;

use crate::module::translate::{task::model::Task, worker::model::Worker};

impl Worker {
    pub fn create(
        worker_id: u16,
        project_id: u32,
        task_index: Arc<Mutex<u32>>,
        task_list: Arc<Mutex<Vec<Task>>>,
    ) -> Worker {
        Worker {
            worker_id,
            project_id,
            task_index,
            task_list,
        }
    }
}
