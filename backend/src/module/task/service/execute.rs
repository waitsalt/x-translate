use rig::{agent::Agent, completion::Prompt, providers::openai::CompletionModel};

use crate::module::{
    model::ServerResult,
    task::{
        self,
        model::{Task, TaskStatus},
    },
};

impl Task {
    pub async fn execute(&mut self, agent: &Agent<CompletionModel>) -> ServerResult<()> {
        // let client = openai::Client::from_url("api_key", "base_url");
        // let agent = client.agent("deepseek-chat").build();
        let target_text = agent.prompt(&self.source_text).await.unwrap();
        self.target_text = target_text;
        self.task_status = TaskStatus::Success;
        Ok(())
    }
}
