use rocket::serde::json::Json;

use mutual_types::{hash_string, LoginRequest};

use crate::database;

#[post("/login", data = "<data>", format = "json")]
pub async fn login(data: Json<LoginRequest>) -> String {
    trace!("Connection to login endpoint");
    let session_id = database::users::get_session_token(&data.email, hash_string(&data.password).as_str()).await.unwrap();
    session_id
}