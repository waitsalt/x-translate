use serde::{Deserialize, Serialize};
// use std::marker::Copy;

use crate::module::model::{BaseLanguage, PartType};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub project_id: u32,                               // 项目id
    pub interface_id: u32,                             // 使用的llm接口id
    pub worker_max_number: u32,                        // 同时执行任务的最大数量
    pub project_name: String,                          // 项目名称
    pub project_status: ProjectStatus,                 // 项目状态
    pub project_desc: String,                          // 项目描述
    pub source_language: BaseLanguage,                 // 原语言
    pub target_language: BaseLanguage,                 // 目标语言
    pub prompt_language: BaseLanguage,                 // 提示词语言 默认为目标语言
    pub term_table_enable: bool,                       // 是否使用术语表
    pub term_table_prompt_language: BaseLanguage,      // 术语表提示词语言 默认为目标语言
    pub term_table_auto_enable: bool,                  // 术语表自动填充
    pub term_table_auto_prompt_language: BaseLanguage, // 术语表自动填充提示词语言 默认为目标语言
    pub part_type: PartType,                           // 分段类型 默认为段落
    pub part_size: u32,                                // 分段大小 line默认为10字 token默认为600
    pub project_create_time: String,                   // 项目创建时间
    pub project_update_time: String,                   // 项目更新时间
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ProjectStatus {
    Wait,    // 等待开始 创建后的默认状态
    Stop,    // 停止
    Doing,   // 执行中
    Success, // 成功
    Error,   // 错误
}
