use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};

use crate::{common::database::database_connect, module::model::ServerResult};

use super::{
    dto::{InterfaceCreatePayload, InterfaceListParam, InterfaceUpdatePayload},
    model::Interface,
};

static EXECUTER: Lazy<&Pool<Sqlite>> = Lazy::new(|| database_connect());

pub async fn insert_one(interface_create_payload: &InterfaceCreatePayload) -> ServerResult<()> {
    let sql = "
        insert into \"interface\" (interface_name, interface_desc, provider, model_name, api_key, base_url)
        values ($1, $2, $3, $4, $5, $6);
    ";
    let _ = sqlx::query(sql)
        .bind(&interface_create_payload.interface_name)
        .bind(&interface_create_payload.interface_desc)
        .bind(&interface_create_payload.provider)
        .bind(&interface_create_payload.model_name)
        .bind(&interface_create_payload.api_key)
        .bind(&interface_create_payload.base_url)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn select_one_by_id(interface_id: u32) -> ServerResult<Interface> {
    let sql = "
        select *
        from \"interface\"
        where id = $1;
    ";
    let result: Interface = sqlx::query_as(sql)
        .bind(interface_id)
        .fetch_one(*EXECUTER)
        .await
        .unwrap();
    Ok(result)
}

pub async fn select_one_by_name(interface_name: &str) -> ServerResult<Interface> {
    let sql = "
        select *
        from \"interface\"
        where interface_name = $1;
    ";
    let result: Interface = sqlx::query_as(sql)
        .bind(interface_name)
        .fetch_one(*EXECUTER)
        .await
        .unwrap();
    Ok(result)
}

pub async fn select_all() -> ServerResult<Vec<Interface>> {
    let sql = "
        select *
        from \"interface\";
    ";
    let result: Vec<Interface> = sqlx::query_as(sql).fetch_all(*EXECUTER).await.unwrap();
    Ok(result)
}

pub async fn update_one_enable_by_id(interface_id: u32, enable: bool) -> ServerResult<()> {
    let sql = "
        update \"interface\"
        where id = $1
        set enable = $2;
    ";
    let _ = sqlx::query(sql)
        .bind(interface_id)
        .bind(enable)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_one_base_info_by_id(
    interface_update_payload: &InterfaceUpdatePayload,
) -> ServerResult<()> {
    let mut sql = "
        update \"interface\"
        where id = $1
        set 1=1
    "
    .to_string();

    if let Some(_interface_name) = &interface_update_payload.interface_name {
        sql.push_str(",interface_name = $2");
    };
    if let Some(_interface_desc) = &interface_update_payload.interface_desc {
        sql.push_str(",interface_desc = $3");
    }
    if let Some(_provider) = &interface_update_payload.provider {
        sql.push_str(",provider = $4");
    }
    if let Some(_model_name) = &interface_update_payload.model_name {
        sql.push_str(",model_name = $5");
    }
    if let Some(_api_key) = &interface_update_payload.api_key {
        sql.push_str(",api_key = $6");
    }
    if let Some(_base_url) = &interface_update_payload.base_url {
        sql.push_str(",base_url = $7");
    }
    let _ = sqlx::query(&sql)
        .bind(interface_update_payload.interface_id)
        .bind(&interface_update_payload.interface_name)
        .bind(&interface_update_payload.interface_desc)
        .bind(&interface_update_payload.provider)
        .bind(&interface_update_payload.model_name)
        .bind(&interface_update_payload.api_key)
        .bind(&interface_update_payload.base_url)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn delete_one_by_id(interface_id: u32) -> ServerResult<()> {
    let sql = "
        delete from \"interface\"
        where id = $1;
    ";
    let _ = sqlx::query(sql)
        .bind(interface_id)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn select_some_by_name(
    interface_list_param: &InterfaceListParam,
) -> ServerResult<Vec<Interface>> {
    let mut sql = "
        select *
        from \"interface\"
        offset $2
        limit $3
    "
    .to_string();
    let keyword = match interface_list_param.keyword.as_ref() {
        Some(keyword) => {
            sql.push_str("where interface_name like \"%$1%\"");
            keyword
        }
        None => "",
    };
    let page = match interface_list_param.page {
        Some(page) => page,
        None => 1,
    };
    let limit = match interface_list_param.limit {
        Some(limit) => limit,
        None => 10,
    };
    let offset = (page - 1) * limit;
    let interface_list: Vec<Interface> = sqlx::query_as(&sql)
        .bind(keyword)
        .bind(offset)
        .bind(limit)
        .fetch_all(*EXECUTER)
        .await
        .unwrap();
    Ok(interface_list)
}
