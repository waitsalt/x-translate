use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};

use crate::{
    common::{database::database_connect, error::AppError},
    module::{self, model::ServerResult},
};

use super::{
    dto::{ProjectCreatePayload, ProjectListParam, ProjectUpdatePayload},
    model::Project,
};

static EXECUTER: Lazy<&Pool<Sqlite>> = Lazy::new(|| database_connect());

pub async fn insert_one(project_create_paylad: &ProjectCreatePayload) -> ServerResult<()> {
    let sql = "
        insert into projects (
            interface_id,
            worker_max_number,
            project_name,
            project_desc,
            source_language,
            target_language,
            prompt_language,
            part_type,
            part_size,
            term_table_enable,
            term_table_prompt_language,
            term_table_auto_enable,
            term_table_auto_prompt_language
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
    ";
    let _ = sqlx::query(sql)
        .bind(project_create_paylad.interface_id)
        .bind(project_create_paylad.worker_max_number)
        .bind(&project_create_paylad.project_name)
        .bind(&project_create_paylad.project_desc)
        .bind(&project_create_paylad.source_language)
        .bind(&project_create_paylad.target_language)
        .bind(&project_create_paylad.prompt_language)
        .bind(&project_create_paylad.part_type)
        .bind(&project_create_paylad.part_size)
        .bind(&project_create_paylad.term_table_enable)
        .bind(&project_create_paylad.term_table_prompt_language)
        .bind(&project_create_paylad.term_table_auto_enable)
        .bind(&project_create_paylad.term_table_auto_prompt_language)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn select_one_by_name(project_name: &str) -> ServerResult<Project> {
    let sql = "
        select *
        from \"project\"
        where project_name = $1
    ";
    let project: Project = sqlx::query_as(sql)
        .bind(project_name)
        .fetch_one(*EXECUTER)
        .await
        .unwrap();
    Ok(project)
}

pub async fn select_one_by_id(project_id: u32) -> ServerResult<Project> {
    let sql = "
        select *
        from \"project\"
        where project_id = $1
    ";
    let project: Project = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_one(*EXECUTER)
        .await
        .unwrap();
    Ok(project)
}

pub async fn delete_one_by_id(project_id: u32) -> ServerResult<()> {
    let sql = "
        delete from \"project\"
        where project_id = $1
        ";
    let _ = sqlx::query(sql)
        .bind(project_id)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}

pub async fn select_some_by_name(
    project_list_param: &ProjectListParam,
) -> ServerResult<Vec<Project>> {
    let mut sql = "
        select *
        from \"project\"
        offset $2
        limit $3
    "
    .to_string();
    let keyword = match project_list_param.keyword.as_ref() {
        Some(keyword) => {
            sql.push_str(" where project_name like \"%$1%\"");
            keyword
        }
        None => "",
    };
    let page = match project_list_param.page {
        Some(page) => page,
        None => 1,
    };
    let limit = match project_list_param.limit {
        Some(limit) => limit,
        None => 10,
    };
    let offset = (page - 1) * limit;
    let project_list: Vec<Project> = sqlx::query_as(&sql)
        .bind(keyword)
        .bind(offset)
        .bind(limit)
        .fetch_all(*EXECUTER)
        .await
        .unwrap();
    Ok(project_list)
}

pub async fn update_one_by_id(
    project_id: u32,
    project_update_payload: &ProjectUpdatePayload,
) -> ServerResult<()> {
    let mut sql = "
        update \"project\"
        where id = $1
        set 1=1
    "
    .to_string();
    if let Some(interface_id) = project_update_payload.interface_id {
        // 确认项目名是否存在
        match module::interface::repository::select_one_by_id(interface_id).await {
            // 存在则修改
            Ok(_) => {
                sql.push_str(",interface_id = $2");
            }
            // 不存在则报错
            Err(_) => {
                return Err(AppError::Other);
            }
        }
    };

    if let Some(_worker_max_number) = &project_update_payload.worker_max_number {
        sql.push_str(",worker_max_number = $3");
    };

    if let Some(project_name) = &project_update_payload.project_name {
        // 确认项目名是否存在
        match select_one_by_name(project_name).await {
            // 存在则报错
            Ok(_) => {
                return Err(AppError::Other);
            }
            // 不存在则修改
            Err(_) => {
                sql.push_str(",project_name = $4");
            }
        }
    };
    if let Some(_project_status) = &project_update_payload.project_status {
        sql.push_str(",project_status = $5");
    };

    if let Some(_project_desc) = &project_update_payload.project_desc {
        sql.push_str(",project_desc = $6");
    };

    if let Some(_source_language) = &project_update_payload.source_language {
        sql.push_str(",source_language = $7");
    };

    if let Some(_target_language) = &project_update_payload.target_language {
        sql.push_str(",target_language = $8");
    };

    if let Some(_prompt_language) = &project_update_payload.prompt_language {
        sql.push_str(",prompt_language = $9");
    };

    if let Some(_part_type) = &project_update_payload.part_type {
        sql.push_str(",part_type = $10");
    };

    if let Some(_part_size) = project_update_payload.part_size {
        sql.push_str(",part_size = $11");
    };

    if let Some(_term_table_enable) = project_update_payload.term_table_enable {
        sql.push_str(",term_table_enable = $12");
    };

    if let Some(_term_table_prompt_language) = &project_update_payload.term_table_prompt_language {
        sql.push_str(",term_table_prompt_language = $13");
    };

    if let Some(_term_table_auto_enable) = &project_update_payload.term_table_auto_enable {
        sql.push_str(",term_table_auto_enable = $14");
    };

    if let Some(_term_table_auto_prompt_language) =
        &project_update_payload.term_table_auto_prompt_language
    {
        sql.push_str(",term_table_auto_prompt_language = $15");
    };

    let _ = sqlx::query(&sql)
        .bind(project_id)
        .bind(project_update_payload.interface_id)
        .bind(project_update_payload.worker_max_number)
        .bind(&project_update_payload.project_name)
        .bind(&project_update_payload.project_status)
        .bind(&project_update_payload.project_desc)
        .bind(&project_update_payload.source_language)
        .bind(&project_update_payload.target_language)
        .bind(&project_update_payload.prompt_language)
        .bind(&project_update_payload.part_type)
        .bind(&project_update_payload.part_size)
        .bind(&project_update_payload.term_table_enable)
        .bind(&project_update_payload.term_table_prompt_language)
        .bind(&project_update_payload.term_table_auto_enable)
        .bind(&project_update_payload.term_table_auto_prompt_language)
        .execute(*EXECUTER)
        .await
        .unwrap();
    Ok(())
}
