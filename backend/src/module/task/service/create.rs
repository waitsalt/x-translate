use crate::module::task::model::{Task, TaskStatus};

impl Task {
    pub async fn create(
        task_id: u32,
        source_text: String,
        target_text: String,
        task_status: TaskStatus,
    ) -> Self {
        Self {
            task_id,
            source_text,
            target_text,
            task_status,
        }
    }
}
