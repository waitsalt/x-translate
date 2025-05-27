use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};

use crate::{common::database::database_connect, module::model::ServerResult};

use super::{dto::ProjectCreatePayload, model::Project};

static EXECUTER: Lazy<&Pool<Sqlite>> = Lazy::new(|| database_connect());

pub async fn insert_one(project_create_paylad: &ProjectCreatePayload) -> ServerResult<()> {
    let sql = "
        insert into projects (
            llm_id,
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
        .bind(project_create_paylad.llm_id)
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

pub async fn get_one_by_name(project_name: &str) -> ServerResult<Project> {
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

pub async fn get_one_by_id(project_id: u32) -> ServerResult<Project> {
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
