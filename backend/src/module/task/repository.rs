use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};

use crate::{common::database::database_connect, module::model::ServerResult};

use super::{dto::TaskCreatePayload, model::Task};

static EXECUTER: Lazy<&Pool<Sqlite>> = Lazy::new(|| database_connect());

pub async fn insert_some(project_id: u32, task_list: &[TaskCreatePayload]) -> ServerResult<()> {
    if task_list.is_empty() {
        return Ok(());
    }

    // 构建批量插入的 SQL 语句
    let mut sql = String::from(
        "insert into \"task\" (project_id, source_text, target_text, task_status) values ",
    );
    let mut placeholders = Vec::new();

    for i in 0..task_list.len() {
        let offset = i * 4;
        placeholders.push(format!(
            "(${}, ${}, ${}, ${})",
            offset + 1,
            offset + 2,
            offset + 3,
            offset + 4
        ));
    }

    sql.push_str(&placeholders.join(", "));

    // 创建查询并绑定所有参数
    let mut query = sqlx::query(&sql);

    for task in task_list {
        query = query
            .bind(project_id)
            .bind(&task.source_text)
            .bind(&task.target_text)
            .bind(&task.task_status);
    }

    let _ = query.execute(*EXECUTER).await.unwrap();

    Ok(())
}

pub async fn select_some_by_project_id(project_id: u32) -> ServerResult<Vec<Task>> {
    let sql = "
        select *
        from \"task\"
        where project_id = $1
        ";
    let task_list = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(*EXECUTER)
        .await
        .unwrap();

    Ok(task_list)
}

pub async fn delete_some_by_project_id(project_id: u32) -> ServerResult<()> {
    let sql = "
        delete from \"task\"
        where project_id = $1
        ";
    let _ = sqlx::query(sql)
        .bind(project_id)
        .execute(*EXECUTER)
        .await
        .unwrap();

    Ok(())
}
