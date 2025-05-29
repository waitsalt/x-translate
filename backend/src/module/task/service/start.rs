use std::sync::Arc;

use rig::agent::Agent;

use crate::module::{
    interface::model::Interface, model::ServerResult, project::model::Project, task::model::Task,
};

pub async fn start(project: &Project, interface: &Interface, task: &Task) -> ServerResult<()> {
    Ok(())
}
