use crate::module::{
    interface::{dto::InterfaceListParam, model::Interface, repository::select_some_by_name},
    model::ServerResult,
};

pub async fn list(interface_list_param: &InterfaceListParam) -> ServerResult<Vec<Interface>> {
    let result = select_some_by_name(interface_list_param).await.unwrap();
    Ok(result)
}
