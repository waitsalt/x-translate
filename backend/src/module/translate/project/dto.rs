use serde::Deserialize;

use super::model::PartType;

#[derive(Debug, Deserialize)]
pub struct ProjectCreatePayload {
    pub llm_id: u32,                             // 使用的模型id
    pub worker_max_number: u16,                  // 同时执行任务的最大数量
    pub project_name: String,                    // 项目名称
    pub project_desc: String,                    // 项目描述
    pub source_language: String,                 // 原语言
    pub target_language: String,                 // 目标语言
    pub prompt_language: String,                 // 提示词语言 默认为目标语言
    pub part_type: PartType,                     // 分段类型 默认为段落
    pub part_size: u32,                          // 分段大小 默认为1000字
    pub term_table_enable: bool,                 // 是否使用术语表
    pub term_table_prompt_language: String,      // 术语表提示词语言 默认为目标语言
    pub term_table_auto_enable: bool,            // 术语表自动填充
    pub term_table_auto_prompt_language: String, // 术语表自动填充提示词语言 默认为目标语言
    pub project_create_time: String,             // 项目创建时间
    pub project_update_time: String,             // 项目更新时间
}
