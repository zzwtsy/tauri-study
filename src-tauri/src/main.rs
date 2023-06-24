// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod commands;
mod dao;
mod db;
mod init;
mod res;
mod service;
mod tools;
mod utils;

use commands::{get_all_wakatime_data, gist_id, greet, query_wakatime_by_date_range};
use init::Init;

fn main() {
    match Init::create_table() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            gist_id,
            get_all_wakatime_data,
            query_wakatime_by_date_range
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
