mod commands;
mod db;
mod services;

use commands::{get_app_version, get_system_info, get_setting, set_setting, save_queue, load_queue};
use commands::music::{
    music_search, music_song_url, music_lyric,
    music_login_status, music_login_cookie, music_logout,
    music_open_web_login, music_cancel_web_login,
    music_qr_key, music_qr_create, music_qr_check,
    music_qq_login_status, music_qq_login_cookie, music_qq_logout,
    music_audio_proxy,
    music_user_playlists, music_playlist_tracks,
    music_like_check, music_like_toggle,
};
use db::Database;
use services::cookie::CookieStore;
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
            let cookies = CookieStore::open(&data_dir);
            app.manage(database);
            app.manage(cookies);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_version, get_system_info, get_setting, set_setting,
            save_queue, load_queue,
            music_search, music_song_url, music_lyric,
            music_login_status, music_login_cookie, music_logout,
            music_open_web_login, music_cancel_web_login,
            music_qr_key, music_qr_create, music_qr_check,
            music_qq_login_status, music_qq_login_cookie, music_qq_logout,
            music_audio_proxy,
            music_user_playlists, music_playlist_tracks,
            music_like_check, music_like_toggle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
