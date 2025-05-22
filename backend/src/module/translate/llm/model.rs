pub struct Llm {
    pub llm_id: u32,
    pub llm_name: String,   // llm 名字
    pub model_name: String, // 模型名
    pub api_key: String,    // llm 密钥
    pub base_url: String,   // llm 调用的基本地址
    pub enable: bool,       // llm 是否可以使用
}
