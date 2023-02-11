#[tauri::command]
pub fn hello_nested() -> Result<String, String> {
    Ok("Hello Nested!".into())
}
