use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Task {
    pub task_id: u32,            // 任务id
    pub project_id: u32,         // 项目id
    pub source_text: String,     // 源文本
    pub target_text: String,     // 目标文本
    pub task_status: TaskStatus, // 任务状态
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    Wait,
    Doing,
    Success,
    Error,
}
