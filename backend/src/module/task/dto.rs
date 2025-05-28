use serde::Deserialize;

use super::model::TaskStatus;

#[derive(Debug, Deserialize)]
pub struct TaskCreatePayload {
    pub source_text: String,
    pub target_text: String,
    pub task_status: TaskStatus,
}
