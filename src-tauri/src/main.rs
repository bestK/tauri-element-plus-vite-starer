#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn http_get(request_url: &str) -> Result<String, String> {
    app::request::get(request_url)
}

#[tauri::command]
fn http_post(request_url: &str, params: &str) -> Result<String, String> {
    app::request::post_json(request_url, params)
}

#[tauri::command]
fn read_hosts(os_type: &str) -> String {
    app::read_hosts_file(os_type)
}

#[tauri::command]
fn write_hosts(os_type: &str, hosts: &str) {
    app::write_hosts_file(os_type, hosts)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            http_get,
            http_post,
            read_hosts,
            write_hosts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
