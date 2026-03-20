#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::Manager;
use tauri::window::Color;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_background_color(Some(Color(0, 0, 0, 0)));
            let _ = window.show();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run_cmd(path: String) -> Result<String, String> {
    match Command::new("cmd").args(["/c", "start", "", &path]).spawn() {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => Err(format!("Failed: {}", e)),
    }
}
