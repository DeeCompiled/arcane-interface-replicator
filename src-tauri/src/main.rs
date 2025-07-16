// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_dialog::DialogExt;
use walkdir::{DirEntry, WalkDir};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![select_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn select_directory(app_handle: tauri::AppHandle) -> String {
    if let Some(path) = app_handle.dialog().file().blocking_pick_folder() {
        let mut game_versions = Vec::new();

        for entry in WalkDir::new(&path.to_string())
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| is_valid_game_folder(e))
        {
            let folder_name = entry.file_name().to_string_lossy().to_string();
            game_versions.push(folder_name);
        }

        return game_versions.join("\n");
    }

    "".to_string()
}

fn is_valid_game_folder(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && !is_hidden(entry)
        && entry
            .file_name()
            .to_str()
            .map(|name| name.starts_with('_') 
                              && name.ends_with('_'))
            .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
