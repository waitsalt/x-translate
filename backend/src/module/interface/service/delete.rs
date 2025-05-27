use crate::module::{interface::repository, model::ServerResult};

pub async fn delete(interface_id: u32) -> ServerResult<()> {
    let _ = repository::delete_one_by_id(interface_id).await.unwrap();
    Ok(())
}
