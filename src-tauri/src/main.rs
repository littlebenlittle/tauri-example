#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request_answer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn request_answer() -> String {
    "Foo!".into()
}
