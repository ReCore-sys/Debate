use rocket::serde::json::Json;
use serde::Deserialize;

use crate::database;

#[derive(Deserialize)]
pub struct LoginRequest {
    uuid: String,
    password: String,
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login(data: Json<LoginRequest>) -> String {
    let session_id = database::users::check_login(&data.uuid, &data.password).await.unwrap();
    session_id
}