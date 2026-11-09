#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod runner;
mod store;

use std::path::PathBuf;
use tauri::window::Color;
use tauri::Manager;

#[tauri::command]
fn load_store(legacy: Option<String>) -> Result<store::StorePayload, String> {
    store::load_store(legacy)
}

#[tauri::command]
fn save_commands(commands: Vec<store::CommandItem>) -> Result<(), String> {
    store::save_commands(commands)
}

#[tauri::command]
fn run_command(id: String) -> Result<(), String> {
    let (mut data, _) = store::read_store()?;
    let item = data
        .commands
        .iter()
        .find(|c| c.id == id)
        .cloned()
        .ok_or_else(|| "找不到该命令，可能已被删除".to_string())?;

    if item.command.trim().is_empty() {
        return Err("命令内容为空，请先编辑这条命令".to_string());
    }

    let workdir = if item.workdir.trim().is_empty() {
        None
    } else {
        let dir = PathBuf::from(item.workdir.trim());
        if !dir.is_dir() {
            return Err(format!("工作目录不存在：{}", dir.display()));
        }
        Some(dir)
    };

    runner::launch(&item.command, workdir.as_deref(), item.run_as_admin)?;

    if let Some(target) = data.commands.iter_mut().find(|c| c.id == id) {
        target.last_run_at = Some(store::now_millis());
    }
    store::write_store(&data)?;
    Ok(())
}

#[tauri::command]
fn open_data_dir() -> Result<(), String> {
    let dir = store::data_dir()?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("创建数据目录失败：{e}"))?;
    runner::open_dir(&dir)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_background_color(Some(Color(0, 0, 0, 0)));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_store,
            save_commands,
            run_command,
            open_data_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
