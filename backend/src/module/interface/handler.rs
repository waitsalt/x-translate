use axum::{Json, extract::Path};

use crate::{common::response::AppResponse, module::model::AppResult};

use super::{
    dto::{InterfaceCreatePayload, InterfaceListParam, InterfaceUpdatePayload},
    model::Interface,
    service,
};

pub async fn create(
    Json(interface_create_payload): Json<InterfaceCreatePayload>,
) -> AppResult<Interface> {
    let interface = service::create(&interface_create_payload).await?;
    Ok(AppResponse::success(Some(interface)))
}

pub async fn delete(Path(interface_id): Path<u32>) -> AppResult<()> {
    let _ = service::delete(interface_id).await?;
    Ok(AppResponse::success(None))
}

pub async fn list(
    Json(interface_list_param): Json<InterfaceListParam>,
) -> AppResult<Vec<Interface>> {
    let interface_list = service::list(&interface_list_param).await?;
    Ok(AppResponse::success(Some(interface_list)))
}

pub async fn info(Path(interface_id): Path<u32>) -> AppResult<Interface> {
    let interface = service::info(interface_id).await?;
    Ok(AppResponse::success(Some(interface)))
}

pub async fn update_base_info(
    Json(interface_update_payload): Json<InterfaceUpdatePayload>,
) -> AppResult<()> {
    let _ = service::update_base_info(&interface_update_payload).await?;
    Ok(AppResponse::success(None))
}

pub async fn test(Json(interface): Json<Interface>) -> AppResult<()> {
    let text = service::test(&interface).await?;
    Ok(AppResponse::success_with_message(text, None))
}
