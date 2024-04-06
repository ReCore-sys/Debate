use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database;
use crate::database::users::User;

#[post("/signup", data = "<data>", format = "json")]
pub async fn signup(data: Json<User>) -> Status {
    match database::users::sign_up(data.into_inner()).await {
        Ok(stat_code) => Status::new(stat_code as u16),
        Err(_) => Status::BadRequest,
    }
}