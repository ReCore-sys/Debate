use reqwest::Client;
use tauri::Error;

use mutual_types::types::StatusResponse;

use crate::config::config::get_config;

#[tauri::command]
pub async fn status() -> Result<Option<StatusResponse>, Error> {
    let client = Client::new();

    let res = client.get(format!("{}/status", get_config().api_host))
        .send()
        .await;

    return match res {
        Ok(res) => Ok(Some(res.json().await.unwrap())),
        Err(e) => {
            eprintln!("{}", e);
            Ok(None)
        }
    };
}