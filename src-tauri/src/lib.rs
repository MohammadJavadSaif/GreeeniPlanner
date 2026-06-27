// GreeniPlanner — Tauri backend
// Minimal by design: all app logic lives in ui/index.html.
// This file just wires Tauri together.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running GreeniPlanner");
}
