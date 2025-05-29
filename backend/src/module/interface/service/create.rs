use crate::{
    common::error::AppError,
    module::{
        interface::{
            dto::InterfaceCreatePayload,
            model::{Interface, Provider},
            repository,
        },
        model::ServerResult,
    },
};

pub async fn create(interface_create_payload: &InterfaceCreatePayload) -> ServerResult<Interface> {
    // 判断base_url是否必要
    if interface_create_payload.provider == Provider::OpenaiCustom
        || interface_create_payload.provider == Provider::AnthropicCustom
    {
        if interface_create_payload.base_url == None {
            return Err(AppError::Other);
        }
    }
    let _ = repository::insert_one(&interface_create_payload).await?;
    let interface =
        repository::select_one_by_name(&interface_create_payload.interface_name).await?;
    Ok(interface)
}
