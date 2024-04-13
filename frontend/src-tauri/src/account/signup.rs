use tauri::{Error, State};

use mutual_types::User;

use crate::GlobalState;

#[tauri::command]
pub async fn signup(user: User, state: State<'_, GlobalState>) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/signup", state.config.api_host))
        .json(&user)
        .send()
        .await;
    return match res {
        Ok(res) => {
            if res.status().is_success() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => {
            if e.is_builder() {}
            eprintln!("{}", e);
            Ok(false)
        }
    };
}