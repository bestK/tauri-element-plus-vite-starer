#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::http_util::{req_get, req_post_json};
use whoami;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// 获取当前电脑用户名
#[tauri::command]
fn get_realname() -> String {
    whoami::realname()
}

#[tauri::command]
fn http_get(request_url: &str) -> Result<String, String> {
    req_get(request_url)
}

#[tauri::command]
fn http_post(request_url: &str, params: &str) -> Result<String, String> {
    req_post_json(request_url, params)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_realname,
            http_get,
            http_post
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
