use rig::{completion::Prompt, providers::openai};

use crate::{
    common::error::AppError,
    module::{interface::model::Interface, model::ServerResult},
};

use super::update;

pub async fn test(interface: &Interface) -> ServerResult<String> {
    let client = openai::Client::from_url(&interface.api_key, &interface.base_url);
    let agent = client.agent(&interface.model_name).build();
    let response = agent.prompt("").await;
    match response {
        Ok(text) => {
            // 原来接口不可以使用就更改
            if interface.enable == false {
                update::update_enable(interface.interface_id, true).await?;
            }
            return Ok(text);
        }
        Err(_) => {
            // 原来接不可以使用就更改
            if interface.enable == true {
                update::update_enable(interface.interface_id, false).await?;
            }
            return Err(AppError::LlmError);
        }
    }
}
