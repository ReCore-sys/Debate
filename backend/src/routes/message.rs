use rocket::http::Status;
use rocket::serde::json::Json;

use mutual_types::Message;

use crate::database::messages::send_message;

#[post("/message/<session>", data = "<data>", format = "json")]
pub async fn message(data: Json<Message>, session: String) -> Status {
    trace!("Connection to message endpoint");
    match send_message(data.into_inner(), session).await {
        Ok(stat_code) => Status::new(stat_code as u16),
        Err(err) => {
            eprintln!("{:?}", err); // Print the error to the console
            Status::BadRequest
        }
    }
}