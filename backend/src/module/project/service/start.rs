use std::path::Path;

use crate::{
    common::error::AppError,
    module::{
        self,
        model::ServerResult,
        project::{model::ProjectStatus, repository},
    },
};

// 当前项目只有当状态为 Wait 时，才能启动项目
pub async fn start(project_id: u32) -> ServerResult<()> {
    let project = repository::select_one_by_id(project_id).await?;

    // 当状态不为 Wait 时，不能启动项目
    match project.project_status {
        ProjectStatus::Doing => {
            return Err(AppError::Other);
        }
        ProjectStatus::Error => {
            return Err(AppError::Other);
        }
        ProjectStatus::Stop => {
            return Err(AppError::Other);
        }
        ProjectStatus::Success => {
            return Err(AppError::Other);
        }
        ProjectStatus::Wait => (),
    };

    // 获取任务
    let mut task_list = module::task::repository::select_some_by_project_id(project_id)
        .await
        .unwrap();

    // 构造用户上传文件的目录
    let project_input_path = Path::new("project")
        .join(project_id.to_string())
        .join("input");

    // 检查任务表是否为空
    let task_list_empty_is = task_list.is_empty();

    //检查用户上传文件目录是否有文件存在
    let input_dir_has_file_is = module::util::has_file_in_dir(&project_input_path)
        .await
        .unwrap();

    // 既没有任务列表，也没有文件 就报错
    if task_list_empty_is && !input_dir_has_file_is {
        return Err(AppError::Other);
    }

    tokio::spawn(async move {
        // 有文件且没有任务就生成
        if task_list_empty_is && input_dir_has_file_is {
            todo!()
        }
        // 启动对应数量的worker解决任务
    });

    Ok(())
}
