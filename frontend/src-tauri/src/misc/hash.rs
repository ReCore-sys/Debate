use mutual_types::hash_string;
use tauri::Error;
#[tauri::command]
pub async fn hash(data: String) -> Result<Option<String>, Error> {
    Ok(Some(hash_string(&data)))
}