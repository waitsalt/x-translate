use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub interface_id: u32,
    pub interface_name: String,   // 接口名字
    pub interface_desc: String,   // 接口描述
    pub provider: Provider,       // llm 提供商
    pub model_name: String,       // llm 模型名
    pub api_key: String,          // llm 密钥
    pub base_url: Option<String>, // llm 调用的基本地址
    pub enable: bool,             // llm 是否可以使用
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum Provider {
    Aliyun,
    Google,
    Openai,
    Sakura,
    Zeroone,
    Zhipuai,
    Deepseek,
    Moonshot,
    Anthropic,
    Volcengine,
    Siliconflow,
    OpenaiCustom,
    AnthropicCustom,
}
