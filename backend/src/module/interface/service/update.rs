use crate::module::{
    interface::{dto::InterfaceUpdatePayload, repository},
    model::ServerResult,
};

pub async fn update_enable(interface_id: u32, enable: bool) -> ServerResult<()> {
    let _ = repository::update_one_enable_by_id(interface_id, enable).await?;
    Ok(())
}

pub async fn update_base_info(
    interface_id: u32,
    interface_update_payload: &InterfaceUpdatePayload,
) -> ServerResult<()> {
    let _ = repository::update_one_base_info_by_id(interface_id, interface_update_payload).await?;
    Ok(())
}
