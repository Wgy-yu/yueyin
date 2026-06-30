use tauri::State;
use crate::db::{Database, QueueTrack};

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_system_info() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    format!("{} ({})", os, arch)
}

#[tauri::command]
pub fn get_setting(db: State<'_, Database>, key: String) -> Result<Option<String>, String> {
    db.get_setting(&key)
}

#[tauri::command]
pub fn set_setting(db: State<'_, Database>, key: String, value: String) -> Result<(), String> {
    db.set_setting(&key, &value)
}

#[tauri::command]
pub fn save_queue(db: State<'_, Database>, tracks: Vec<QueueTrack>) -> Result<(), String> {
    db.save_queue(&tracks)
}

#[tauri::command]
pub fn load_queue(db: State<'_, Database>) -> Result<Vec<QueueTrack>, String> {
    db.load_queue()
}
