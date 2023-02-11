#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn test_call() -> Result<String, String> {
  Ok("Hello Rust!".into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test_call])  
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
