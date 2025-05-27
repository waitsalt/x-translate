use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub interface_id: u32,
    pub interface_name: String, // 接口名字
    pub interface_desc: String, // 接口藐视
    pub provider: Provider,     // llm 提供商
    pub model_name: String,     // llm 模型名
    pub api_key: String,        // llm 密钥
    pub base_url: String,       // llm 调用的基本地址
    pub enable: bool,           // llm 是否可以使用
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum Provider {
    OpenAI,
    DeepSeek,
    Gemini,
    ChatGPT,
    ChatGLM,
    Other,
}
