use rand;
use serde::{Deserialize, Serialize};
use uuid;

use crate::database::connect;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub pfp: String,
    pub email: String,
    pub bio: String,
}

pub async fn gen_random_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let session_id: String = (0..32).map(|_| rng.gen_range(0..16).to_string()).collect();
    session_id
}

pub async fn check_login(uuid: &str, password: &str) -> Result<String, surrealdb::Error> {
    let conn = connect().await?;

    let mut query = "SELECT * FROM users WHERE uuid = $uuid AND password = $password";

    let mut result = conn.query(query).bind(("uuid", uuid)).bind(("password", password)).await?;
    return if result.take::<Vec<User>>(0)?.len() == 1 {
        query = "DELETE FROM sessions WHERE uuid = $uuid";
        conn.query(query).bind(("uuid", uuid)).await?;
        let session_id = gen_random_session_id().await;
        let session_id = session_id.as_str();
        query = "INSERT INTO sessions (session_id, uuid) VALUES ($session_id, $uuid)";
        conn.query(query).bind(("session_id", session_id)).bind(("uuid", uuid)).await?;
        Ok(session_id.to_string())
    } else {
        Ok("".to_string())
    };
}

pub async fn user_from_uuid(uuid: &str) -> Result<Option<User>, surrealdb::Error> {
    let conn = connect().await?;

    let query = "SELECT * FROM users WHERE uuid = $uuid";

    let mut result = conn.query(query).bind(("uuid", uuid)).await?.take::<Vec<User>>(0)?;
    if result.len() == 1 {
        match result.pop() {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub async fn user_from_session_id(session_id: &str) -> Result<Option<User>, surrealdb::Error> {
    let conn = connect().await?;

    let query = "SELECT uuid FROM sessions WHERE session_id = $session_id";

    let mut result = conn.query(query).bind(("session_id", session_id)).await?.take::<Vec<String>>(0)?;
    if result.len() == 1 {
        let uuid = result.pop().unwrap();
        user_from_uuid(&uuid).await
    } else {
        Ok(None)
    }
}

pub async fn validate_session_id(session_id: &str, uuid: String) -> Result<bool, surrealdb::Error> {
    let conn = connect().await?;

    let query = "SELECT VALUE session_id FROM sessions WHERE session_id = $session_id AND uuid = $uuid";

    let result = conn.query(query).bind(("session_id", session_id)).bind(("uuid", uuid)).await?.take::<Vec<String>>(0)?;
    Ok(result.len() == 1)
}


pub async fn sign_up(user: User) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;

    let newuser = User {
        uuid: uuid::Uuid::new_v4().to_string(),
        pfp: "".to_string(),
        bio: "".to_string(),
        ..user
    };

    let mut result = conn.query("SELECT * FROM users WHERE email = $email").bind(("email", &newuser.email)).await?;

    if result.take::<Vec<User>>(0)?.len() > 0 {
        return Ok(405);
    }

    conn.query("INSERT INTO users $user").bind(("user", newuser)).await?;
    Ok(200)
}