use crate::database;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<data>", format = "json")]
pub fn login(data: Json<LoginRequest>) -> String {
    let session_id = database::users::check_login(&data.username, &data.password).unwrap();
    session_id
}
