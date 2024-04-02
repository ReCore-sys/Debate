use rand;
use rusqlite;

pub struct User {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub pfp: String,
}

pub fn gen_random_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let session_id: String = (0..32).map(|_| rng.gen_range(0..16).to_string()).collect();
    session_id
}

pub fn check_login(username: &str, password: &str) -> Result<String, rusqlite::Error> {
    let conn = rusqlite::Connection::open("database.db")?;
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ? AND password = ?")?;
    let mut rows = stmt.query(rusqlite::params![username, password])?;
    if let Some(row) = rows.next()? {
        let uuid: String = row.get(2)?;
        let session_id = gen_random_session_id();
        let creation_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let insert_session = conn.execute(
            "INSERT INTO sessions (uuid, session_id, creation_time) VALUES (?, ?, ?)",
            [uuid, session_id.to_string(), creation_time.to_string()],
        )?;
        if insert_session == 1 {
            Ok(session_id)
        } else {
            Ok("nope".to_string())
        }
    } else {
        Ok("nope".to_string())
    }
}

pub async fn user_from_uuid(uuid: &str) -> Result<Option<User>, rusqlite::Error> {
    let conn = rusqlite::Connection::open("database.db")?;
    let mut stmt = conn.prepare("SELECT * FROM users WHERE uuid = ?")?;
    let mut rows = stmt.query(rusqlite::params![uuid])?;
    if let Some(row) = rows.next()? {
        Ok(Some(User {
            username: row.get(0)?,
            password: row.get(1)?,
            uuid: row.get(2)?,
            pfp: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}

pub async fn user_from_session_id(session_id: &str) -> Result<Option<User>, rusqlite::Error> {
    let conn = rusqlite::Connection::open("database.db")?;
    let mut stmt = conn.prepare("SELECT * FROM sessions WHERE session_id = ?")?;
    let mut rows = stmt.query(rusqlite::params![session_id])?;
    if let Some(row) = rows.next()? {
        let uuid: String = row.get(1)?;
        user_from_uuid(&uuid).await
    } else {
        Ok(None)
    }
}
