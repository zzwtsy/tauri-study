use std::collections::HashMap;

use crate::{service::WakaTimeService, utils::HttpUtils};
use entity::wakatime::*;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn gist_id(id: i32) -> String {
    println!("{}", id);

    let url = "https://gist.githubusercontent.com/zzwtsy/070bce177f3e0bc2b012be8ecfa166e4/raw/8168f5580c76be3e15b303fcedac78683dc1c3b0/summaries_2023-01-01.json";

    let header = HashMap::new();

    let res = HttpUtils::get_json::<WakaTimeJsonVec>(url, header).await;

    let json = res.unwrap();

    let res = WakaTimeService::add_wakatime_data(json).await;

    if res.is_err() {
        return res.err().unwrap().to_string();
    } else {
        return "success".to_string();
    }
}
