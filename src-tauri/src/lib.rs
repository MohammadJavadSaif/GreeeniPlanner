// GreeniPlanner — Tauri backend
// Minimal by design: all app logic lives in ui/index.html.
// This file just wires Tauri together. No plugins, no custom commands —
// the app is 100% self-contained HTML/CSS/JS using only localStorage.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running GreeniPlanner");
}
