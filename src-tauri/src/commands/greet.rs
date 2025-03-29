use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn greet(name: &str, state: State<'_, AppState>) -> Result<String, String> {
    let greeting = format!("Hello, {}! You've been greeted from Rust!", name);
    Ok(greeting)
}
