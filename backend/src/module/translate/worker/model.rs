use std::sync::Arc;

use tokio::sync::Mutex;

use crate::module::translate::task::model::Task;

pub struct Worker {
    pub worker_id: u16,
    pub project_id: u32,
    pub task_index: Arc<Mutex<u32>>,
    pub task_list: Arc<Mutex<Vec<Task>>>,
}
