use crate::module::{
    interface::{model::Interface, repository},
    model::ServerResult,
};

pub async fn info(interface_id: u32) -> ServerResult<Interface> {
    let interface = repository::select_one_by_id(interface_id).await?;
    Ok(interface)
}
