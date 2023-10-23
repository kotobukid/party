// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scraping;

use dotenv::dotenv;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet() -> Vec<scraping::EventDetail> {
    dotenv().ok();

    let key = "WX_SEL";
    let root_selectors: Vec<String> = dotenv::var(key).unwrap_or("fukuoka".to_string()).split(',').into_iter().map(|s| s.trim().to_owned()).collect();

    scraping::use_cache_or_fetch(root_selectors).await.unwrap()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
