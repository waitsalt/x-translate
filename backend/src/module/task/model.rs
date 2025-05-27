use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task_id: u32,
    pub source_text: String,
    pub target_text: String,
    pub task_status: TaskStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    Wait,
    Doing,
    Success,
    Error,
}
