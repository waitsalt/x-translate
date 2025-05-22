use tokio::{fs, io::AsyncWriteExt};

use crate::module::{model::ServerResult, translate::project::dto::ProjectCreatePayload};

pub async fn create(project_create_payload: ProjectCreatePayload) -> ServerResult<()> {
    Ok(())
}
