use rand::Rng;

use mutual_types::User;

use crate::database::connect;

/// Generate a 32 character random hex string.
///
/// This function generates a random session ID, which is a 32 character long
/// hexadecimal string. It uses the `rand` crate to generate random numbers,
/// which are then formatted as hexadecimal and collected into a string.

pub async fn gen_random_session_id() -> String {
    let mut rng = rand::thread_rng();
    (0..32).map(|_| format!("{:x}", rng.gen::<u8>())).collect()
}

/// Get a session token for a user.
///
/// This function retrieves a session token for a user identified by their UUID
/// and password. If the user is found in the database, a new session is created
/// and the session ID is returned. If the user is not found, an empty string is
/// returned.
pub async fn get_session_token(username: &str, password: &str) -> Result<String, surrealdb::Error> {
    let conn = connect().await?;

    let mut query = "SELECT * FROM users WHERE username = $username AND password = $password";

    let mut result = conn.query(query).bind(("username", username)).bind(("password", password)).await?;
    return if result.take::<Vec<User>>(0)?.len() == 1 {
        let uuid = user_from_username(username).await?.unwrap().uuid;
        query = "DELETE FROM sessions WHERE uuid = $uuid";
        conn.query(query).bind(("uuid", &uuid)).await?;
        let session_id = gen_random_session_id().await;
        let session_id = session_id.as_str();
        query = "INSERT INTO sessions (session_id, uuid) VALUES ($session_id, $uuid)";
        conn.query(query).bind(("session_id", session_id)).bind(("uuid", uuid)).await?;
        Ok(session_id.to_string())
    } else {
        Ok("".to_string())
    };
}

/// Get a user from their UUID.
///
/// This function retrieves a user from the database using their UUID. If the
/// user is found, they are returned. If the user is not found, `None` is
/// returned.
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

/// Get a user from their session ID.
///
/// This function retrieves a user from the database using their session ID. If
/// the user is found, they are returned. If the user is not found, `None` is
/// returned.
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

pub async fn user_from_username(username: &str) -> Result<Option<User>, surrealdb::Error> {
    let conn = connect().await?;

    let query = "SELECT * FROM users WHERE username = $username";

    let mut result = conn.query(query).bind(("username", username)).await?.take::<Vec<User>>(0)?;
    if result.len() == 1 {
        match result.pop() {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/// Validate a session ID.
///
/// This function validates a session ID by checking if it exists in the
/// database and is associated with the provided UUID. If the session ID is
/// valid, `true` is returned. If the session ID is not valid, `false` is
/// returned.
pub async fn validate_session_id(session_id: &str, uuid: String) -> Result<bool, surrealdb::Error> {
    let conn = connect().await?;

    let query = "SELECT VALUE session_id FROM sessions WHERE session_id = $session_id AND uuid = $uuid";

    let result = conn.query(query).bind(("session_id", session_id)).bind(("uuid", uuid)).await?.take::<Vec<String>>(0)?;
    Ok(result.len() == 1)
}

/// Sign up a new user.
///
/// This function creates a new user in the database. The user's UUID is
/// generated automatically, and their profile picture and bio are set to empty
/// strings. If a user with the same email already exists in the database, the
/// function returns `405`. If the user is created successfully, the function
/// returns `200`.
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