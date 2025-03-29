// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use http::HeaderMap;
use reqwest;
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

// Command to manually check for updates
#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> Result<String, String> {
    println!("Starting update check...");

    // Debug: Try direct fetch first
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let client = reqwest::Client::new();
        let url =
            "https://github.com/dominara-team/dominara-dev/releases/latest/download/latest.json";

        match client.get(url).send().await {
            Ok(response) => {
                println!("Response status: {}", response.status());
                match response.text().await {
                    Ok(text) => println!("Response body: {}", text),
                    Err(e) => println!("Failed to get response body: {}", e),
                }
            }
            Err(e) => println!("Failed to fetch: {}", e),
        }
    }

    let updater = app.updater().map_err(|e| {
        println!("Failed to get updater: {}", e);
        e.to_string()
    })?;

    println!("Checking for updates...");
    match updater.check().await {
        Ok(Some(update)) => {
            println!("Update found: {}", update.version);
            // Start the update installation
            if let Err(e) = update
                .download_and_install(
                    |chunk, total| {
                        println!("Downloaded chunk: {} of {}", chunk, total.unwrap_or(0))
                    },
                    || println!("Download finished, installing update..."),
                )
                .await
            {
                println!("Failed to install update: {}", e);
                return Err(format!("Failed to install update: {}", e));
            }
            Ok(format!("Installing update {}", update.version))
        }
        Ok(None) => {
            println!("No updates available");
            Ok("No updates available".to_string())
        }
        Err(e) => {
            println!("Error checking for updates: {}", e);
            println!("Error details: {:?}", e);
            if let Ok(token) = std::env::var("GITHUB_TOKEN") {
                println!("GitHub token is set (length: {})", token.len());
                println!("Token value: {}", token);
            } else {
                println!("GitHub token is NOT set!");
            }
            Err(format!("Failed to check for updates: {}", e))
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let handle = app.handle();
                let mut headers = HeaderMap::new();
                if let Ok(token) = std::env::var("GITHUB_TOKEN") {
                    println!("GitHub token found, setting up headers...");
                    println!("Token length: {}", token.len());
                    headers.insert(
                        "Authorization",
                        format!("Bearer {}", token).parse().unwrap(),
                    );
                    headers.insert("Accept", "*/*".parse().unwrap());
                    headers.insert("User-Agent", "Tauri Updater".parse().unwrap());
                    println!("Headers set up successfully");
                } else {
                    eprintln!("Warning: GITHUB_TOKEN environment variable not set");
                }
                let updater = tauri_plugin_updater::Builder::new()
                    .headers(headers)
                    .build();
                handle.plugin(updater)?;

                // Initial update check
                let app_handle = handle.clone();
                tauri::async_runtime::spawn(async move {
                    println!("Starting initial update check...");
                    if let Ok(updater) = app_handle.updater() {
                        println!("Got updater instance");
                        match updater.check().await {
                            Ok(Some(update)) => {
                                println!("Update found: {}", update.version);
                                if let Err(e) = update
                                    .download_and_install(
                                        |chunk, total| {
                                            println!(
                                                "Downloaded chunk: {} of {}",
                                                chunk,
                                                total.unwrap_or(0)
                                            )
                                        },
                                        || println!("Download finished, installing update..."),
                                    )
                                    .await
                                {
                                    eprintln!("Failed to install update: {}", e);
                                } else {
                                    println!("Installing update {}", update.version);
                                }
                            }
                            Ok(None) => println!("No updates available"),
                            Err(e) => {
                                eprintln!("Error checking for updates: {}", e);
                                eprintln!("Error details: {:?}", e);
                            }
                        }
                    } else {
                        eprintln!("Failed to get updater instance");
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
