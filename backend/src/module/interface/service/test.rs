use rig::message::Message;

use crate::module::{interface::model::Interface, model::ServerResult};

use super::chat;

pub async fn test(interface: &Interface) -> ServerResult<String> {
    let history: Vec<Message> = vec![Message::user("text"), Message::assistant("text")];
    let response = chat::chat(&interface, "你好", history).await?;
    Ok(response)
}
