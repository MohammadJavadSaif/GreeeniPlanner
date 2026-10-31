// GreeniPlanner — Tauri backend
// Wires Tauri v2 together with the native Notification plugin so ToDo reminders
// appear in the phone/OS system notification center.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("error while running GreeniPlanner");
}
