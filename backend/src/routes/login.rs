use rocket::serde::json::Json;
use mutual_types::LoginRequest;

use crate::database;



#[post("/login", data = "<data>", format = "json")]
pub async fn login(data: Json<LoginRequest>) -> String {
    let session_id = database::users::get_session_token(&data.username, &data.password).await.unwrap();
    session_id
}