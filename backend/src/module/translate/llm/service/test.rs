use rig::{completion::Prompt, providers::openai};

use crate::{
    common::{error::AppError, response::AppResponse},
    module::{model::AppResult, translate::llm::model::Llm},
};

impl Llm {
    pub async fn test(&mut self) -> AppResult<String> {
        let client = openai::Client::from_url(&self.api_key, &self.base_url);
        let agent = client.agent(&self.model_name).build();
        let response = agent.prompt("").await;
        match response {
            Ok(text) => {
                self.enable = true;
                return Ok(AppResponse::success(Some(text)));
            }
            Err(_) => {
                self.enable = false;
                return Err(AppError::LlmError);
            }
        }
    }
}
