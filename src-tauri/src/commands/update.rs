use crate::AppState;
use tauri::{Manager, State};
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
pub async fn check_update(app: tauri::AppHandle) -> Result<String, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => Ok(format!(
            "Update available! Current version: {}, Latest version: {}",
            update.current_version, update.version
        )),
        Ok(None) => Ok("No updates available.".to_string()),
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
    }
}

#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    if let Ok(Some(update)) = updater.check().await {
        update
            .download_and_install(|_, _| {}, || {})
            .await
            .map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}
