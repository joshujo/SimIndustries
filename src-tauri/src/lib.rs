// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod interface;
pub mod core;
pub mod tokio;
pub mod logic;
use parking_lot::Mutex;

use interface::{register::register, retrieve_data::{retrieve_player_assets, unsub_retrieve_player_assets}};

use crate::{ interface::{register::Register, retrieve_data::RetrieveData}, logic::logic };

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            std::thread::spawn(move || {
                logic(app_handle);
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(Register::new())
        .manage(Mutex::new(RetrieveData::default()))
        .invoke_handler(tauri::generate_handler![
            greet, 
            register,
            retrieve_player_assets,
            unsub_retrieve_player_assets
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
