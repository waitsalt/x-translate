use rig::{agent::Agent, providers::openai::CompletionModel};

use crate::module::{
    model::ServerResult,
    {task::model::Task, worker::model::Worker},
};

impl Worker {
    pub async fn execute(&self, agent: &Agent<CompletionModel>) -> ServerResult<()> {
        loop {
            // 获取任务列表长度
            let task_list_length: usize;
            {
                let task_list_guard = self.task_list.lock().await;
                task_list_length = task_list_guard.len();
            }

            // 获取要执行的任务id
            let task_index: u32;
            {
                let mut task_index_guard = self.task_index.lock().await;
                task_index = *task_index_guard;
                *task_index_guard += 1;
            }

            // 当任务id大于列表长度使 退出
            if task_list_length <= (task_index as usize) {
                break;
            }

            // 获取可修改任务
            let mut task: Task;
            {
                let task_list_guard = self.task_list.lock().await;
                task = task_list_guard[task_index as usize].clone();
            }

            // 执行任务
            task.execute(agent).await.unwrap();

            // 将任务保存至任务列表
            {
                let mut task_list_guard = self.task_list.lock().await;
                task_list_guard[task_index as usize] = task;
            }
        }
        Ok(())
    }
}
