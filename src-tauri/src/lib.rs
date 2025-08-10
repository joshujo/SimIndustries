// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod interface;
pub mod core;
pub mod server;
pub mod logic;

use interface::{register::register, retrieve_data::*};

use crate::{ interface::{register::Register, retrieve_data::RetrieveData}, logic::logic, server::channels::server };

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
            server();

            std::thread::spawn(move || {
                logic(app_handle);
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(Register::new())
        .manage(RetrieveData::default())
        .invoke_handler(tauri::generate_handler![
            greet, 
            register,
            retrieve_player_assets,
            unsub_retrieve_player_assets,
            retrieve_inventory_data,
            unsub_retrieve_inventory_data,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
