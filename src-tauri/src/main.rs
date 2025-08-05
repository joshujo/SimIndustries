// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tokio;

fn main() {
    let rt = tokio::runtime::Builder
        ::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio got nuked");
    simindustries_lib::run();

}
