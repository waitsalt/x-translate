use serde::Deserialize;

use super::model::Provider;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceCreatePayload {
    pub interface_name: String, // 接口名字
    pub interface_desc: String, // 接口藐视
    pub provider: Provider,     // llm 提供商
    pub model_name: String,     // llm 模型名
    pub api_key: String,        // llm 密钥
    pub base_url: String,       // llm 调用的基本地址
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceUpdatePayload {
    pub interface_name: Option<String>, // 接口名字
    pub interface_desc: Option<String>, // 接口藐视
    pub provider: Option<Provider>,     // llm 提供商
    pub model_name: Option<String>,     // llm 模型名
    pub api_key: Option<String>,        // llm 密钥
    pub base_url: Option<String>,       // llm 调用的基本地址
}

#[derive(Debug, Deserialize)]
pub struct InterfaceListParam {
    pub keyword: Option<String>, // 接口名字
    pub page: Option<u32>,       // 页码
    pub limit: Option<u32>,      // 每页数量
}

#[derive(Debug, Deserialize)]
pub struct InterfaceSearchParam {
    pub keyword: Option<String>, // 接口名字
    pub page: Option<u32>,       // 页码
    pub limit: Option<u32>,      // 每页数量
}
