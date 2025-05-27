use crate::module::{
    interface::{dto::InterfaceCreatePayload, model::Interface, repository},
    model::ServerResult,
};

pub async fn create(interface_create_payload: &InterfaceCreatePayload) -> ServerResult<Interface> {
    let _ = repository::insert_one(&interface_create_payload).await?;
    let interface =
        repository::select_one_by_name(&interface_create_payload.interface_name).await?;
    Ok(interface)
}
