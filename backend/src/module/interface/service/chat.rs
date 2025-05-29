use rig::{
    agent::Agent,
    completion::Chat,
    message::Message,
    providers::{anthropic, deepseek, gemini, openai},
};

use crate::{
    common::error::AppError,
    module::{
        interface::model::{Interface, Provider},
        model::ServerResult,
    },
};

pub async fn chat(
    interface: &Interface,
    preamble: &str,
    prompt: &str,
    history: Vec<Message>,
) -> ServerResult<String> {
    let response = match interface.provider {
        Provider::Aliyun => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(
                    &interface.api_key,
                    "https://dashscope.aliyuncs.com/compatible-mode/v1",
                )
                .agent(&interface.model_name)
                .preamble(preamble)
                .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Google => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                gemini::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                gemini::Client::new(&interface.api_key)
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Openai => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::new(&interface.api_key)
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Sakura => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(&interface.api_key, "http://localhost:5000/v1")
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Zeroone => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(&interface.api_key, "https://api.lingyiwanwu.com/v1")
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Zhipuai => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(&interface.api_key, "https://open.bigmodel.cn/api/paas/v4")
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Deepseek => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                deepseek::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                deepseek::Client::new(&interface.api_key)
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Moonshot => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(&interface.api_key, "https://api.moonshot.cn/v1")
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Anthropic => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                anthropic::ClientBuilder::new(&interface.api_key)
                    .base_url(&base_url)
                    .build()
                    .agent(&interface.model_name)
                    .build()
            } else {
                anthropic::ClientBuilder::new(&interface.api_key)
                    .build()
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Volcengine => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(
                    &interface.api_key,
                    "https://ark.cn-beijing.volces.com/api/v3",
                )
                .agent(&interface.model_name)
                .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::Siliconflow => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                openai::Client::from_url(&interface.api_key, "https://api.siliconflow.cn/v1")
                    .agent(&interface.model_name)
                    .build()
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::OpenaiCustom => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                openai::Client::from_url(&interface.api_key, &base_url)
                    .agent(&interface.model_name)
                    .build()
            } else {
                return Err(AppError::Other);
            };
            agent.chat(prompt, history).await.unwrap()
        }
        Provider::AnthropicCustom => {
            let agent = if let Some(base_url) = interface.base_url.as_ref() {
                anthropic::ClientBuilder::new(&interface.api_key)
                    .base_url(&base_url)
                    .build()
                    .agent(&interface.model_name)
                    .build()
            } else {
                return Err(AppError::Other);
            };
            agent.chat(prompt, history).await.unwrap()
        }
    };
    Ok(response)
}
