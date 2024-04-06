use serde::{Deserialize, Serialize};

use crate::database::connect;
use crate::database::users::validate_session_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub author: String,
    pub content: String,
    pub timestamp: String,
    pub channel: String,
}

pub async fn send_message(message: Message, session_id: String) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;

    if !validate_session_id(&session_id, message.author.clone()).await.unwrap() {
        return Ok(401);
    }

    let query = "CREATE messages CONTENT $message";

    conn.query(query).bind(("message", message)).await?;

    Ok(200)
}