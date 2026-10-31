// GreeniPlanner — Tauri backend
// Wires Tauri v2 together with the native Notification, Dialog, and FileSystem
// plugins so ToDo reminders and cross-platform JSON Backup & Restore work
// seamlessly across Android, Windows, and macOS.

#[tauri::command]
fn save_backup_to_path(path: String, content: String) -> Result<String, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("Empty file path".to_string());
    }
    let final_path = if trimmed.to_lowercase().ends_with(".json") {
        trimmed.to_string()
    } else {
        format!("{}.json", trimmed)
    };
    let p = std::path::Path::new(&final_path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    std::fs::write(p, content.as_bytes()).map_err(|e| e.to_string())?;
    Ok(final_path)
}

#[tauri::command]
fn read_backup_from_path(path: String) -> Result<String, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("Empty file path".to_string());
    }
    std::fs::read_to_string(trimmed).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            save_backup_to_path,
            read_backup_from_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running GreeniPlanner");
}
