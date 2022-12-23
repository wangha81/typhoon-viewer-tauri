#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json};
pub mod preference;
pub mod typhoon;

// const UPDTAE_DURATION: u64 = 86400 * 14; // 2 weeks
const UPDTAE_DURATION: u64 = 1; // 1 sec debug

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_preference() -> String {
    preference::get().to_string()
}

#[tauri::command]
async fn sync_typhoon_data() -> bool {
    let mut _preference = preference::get();
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if _preference.get("lastUpdate") == None
        || (current_epoch - _preference.get("lastUpdate").unwrap().as_u64().unwrap())
            > UPDTAE_DURATION
    {
        let _res = typhoon::sync().await;

        let now = SystemTime::now();
        let epoch_seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        _preference["lastUpdate"] = json!(epoch_seconds);
        preference::set(_preference);
        true
    } else {
        println!(
            "Typhoon data fresh enough. ({})",
            _preference.get("lastUpdate").unwrap()
        );
        false
    }
}

#[tauri::command]
async fn get_typhoon_data() -> String {
    match typhoon::get() {
        Ok(json) => json.to_string(),
        Err(_) => json!([]).to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            sync_typhoon_data,
            get_typhoon_data,
            get_preference,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
