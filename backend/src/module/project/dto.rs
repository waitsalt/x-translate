use serde::{Deserialize, Serialize};

use crate::module::model::{BaseLanguage, PartType};

use super::model::ProjectStatus;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCreatePayload {
    pub interface_id: u32,                             // 使用的模型id
    pub worker_max_number: u32,                        // 同时执行任务的最大数量
    pub project_name: String,                          // 项目名称
    pub project_desc: String,                          // 项目描述
    pub source_language: BaseLanguage,                 // 原语言
    pub target_language: BaseLanguage,                 // 目标语言
    pub prompt_language: BaseLanguage,                 // 提示词语言 默认为目标语言
    pub part_type: PartType,                           // 分段类型 默认为段落
    pub part_size: u32,                                // 分段大小 默认为1000字
    pub term_table_enable: bool,                       // 是否使用术语表
    pub term_table_prompt_language: BaseLanguage,      // 术语表提示词语言 默认为目标语言
    pub term_table_auto_enable: bool,                  // 术语表自动填充
    pub term_table_auto_prompt_language: BaseLanguage, // 术语表自动填充提示词语言 默认为目标语言
}

#[derive(Debug, Deserialize)]
pub struct ProjectListParam {
    pub keyword: Option<String>, // 项目名字
    pub page: Option<u32>,       // 页码
    pub limit: Option<u32>,      // 每页数量
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdatePayload {
    pub interface_id: Option<u32>,                        // 使用的模型id
    pub worker_max_number: Option<u32>,                   // 同时执行任务的最大数量
    pub project_name: Option<String>,                     // 项目名称
    pub project_status: Option<ProjectStatus>,            // 项目状态
    pub project_desc: Option<String>,                     // 项目描述
    pub source_language: Option<BaseLanguage>,            // 原语言
    pub target_language: Option<BaseLanguage>,            // 目标语言
    pub prompt_language: Option<BaseLanguage>,            // 提示词语言 默认为目标语言
    pub part_type: Option<PartType>,                      // 分段类型 默认为段落
    pub part_size: Option<u32>,                           // 分段大小 line默认为10字 token默认为600
    pub term_table_enable: Option<bool>,                  // 是否使用术语表
    pub term_table_prompt_language: Option<BaseLanguage>, // 术语表提示词语言 默认为目标语言
    pub term_table_auto_enable: Option<bool>,             // 术语表自动填充
    pub term_table_auto_prompt_language: Option<BaseLanguage>, // 术语表自动填充提示词语言 默认为目标语言
}
