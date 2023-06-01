// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod dao;
mod db;
mod init;
mod res;
mod service;
mod tools;
mod utils;
mod api;

use commands::{gist_id, greet};
use init::Init;

fn main() {
    match Init::create_table() {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, gist_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
