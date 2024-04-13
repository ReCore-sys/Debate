use reqwest::Client;
use tauri::{Error, State};

use mutual_types::LoginRequest;

use crate::config::config::get_config;
use crate::GlobalState;

#[tauri::command]
pub async fn login(email: String, password: String, state: State<'_, GlobalState>) -> Result<Option<String>, Error> {
    let login_request = LoginRequest {
        email,
        password,
    };
    let client = Client::new();
    let res = client.post(format!("{}/login", get_config().api_host))
        .json(&login_request)
        .send()
        .await;
    return match res {
        Ok(res) => {
            let token = res.text().await.unwrap();
            let mut state_token = state.session_id.lock().unwrap();
            *state_token = Some(token.clone());
            Ok(Some(token))
        }
        Err(e) => {
            if e.is_builder() {}
            eprintln!("{}", e);
            Ok(Some("err".to_string()))
        }
    };
}