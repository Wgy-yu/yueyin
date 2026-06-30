mod commands;
mod db;

use commands::{get_app_version, get_system_info, get_setting, set_setting};
use db::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()
                .expect("无法获取应用数据目录");
            let database = Database::open(&data_dir)
                .expect("无法初始化数据库");
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_version, get_system_info, get_setting, set_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
