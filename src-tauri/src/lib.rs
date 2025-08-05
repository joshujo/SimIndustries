// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod new_game;
pub mod core;
pub mod tokio;
pub mod logic;
use new_game::register;

use crate::{logic::logic, new_game::Register};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
   

            std::thread::spawn(move || {
                logic(app_handle);
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(Register::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            register
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
