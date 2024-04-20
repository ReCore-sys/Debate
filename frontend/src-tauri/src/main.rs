// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use crate::config::config::Config;

mod account {
    pub mod login;
    pub mod signup;
}

pub mod config {
    pub mod config;
}

pub mod misc {
    pub mod status;
    pub mod hash;
}

pub struct GlobalState {
    pub session_id: Mutex<Option<String>>,
    pub logged_in: AtomicBool,
    pub config: Config,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            session_id: Mutex::new(None),
            logged_in: AtomicBool::new(false),
            config: config::config::get_config(),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(GlobalState::default())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            account::login::login,
            account::signup::signup,
            misc::status::status,
            misc::hash::hash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
