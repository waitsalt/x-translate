use tokio::fs;

use crate::module::translate::llm::model::Llm;

impl Llm {
    pub async fn create(
        llm_id: u32,
        llm_name: String,
        model_name: String,
        api_key: String,
        base_url: String,
    ) -> Self {
        let llm_list_string = fs::read_to_string("path_to_llm_list_file").await.unwrap();
        Llm {
            llm_id,
            llm_name,
            model_name,
            api_key,
            base_url,
            enable: false,
        }
    }
}
