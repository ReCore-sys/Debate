use mutual_types::Message;

use crate::database::connect;
use crate::database::users::validate_session_id;

pub async fn send_message(message: Message, session_id: String) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;

    if !validate_session_id(&session_id, message.author.clone()).await.unwrap() {
        return Ok(401);
    }

    let query = "CREATE messages CONTENT $message";

    conn.query(query).bind(("message", message)).await?;

    Ok(200)
}