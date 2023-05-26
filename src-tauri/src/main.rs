// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod dao;
mod entity;
mod init;
mod tools;
mod utils;

use commands::{gist_id, greet};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, gist_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
