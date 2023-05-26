use std::path::Path;

use crate::dao::wakatime_dao::WakaTimeDao;
use crate::entity::Range;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn gist_id(id: i32) -> String {
    println!("id = {}", id);

    let path = Path::new(".");
    println!("path = {}", path.display());

    let range = Range {
        id,
        date: "2023-04-04".to_string(),
        end: "2023-04-04T15:59:59Z".to_string(),
        start: "2023-04-03T16:00:00Z".to_string(),
        text: "Tue Apr 4th 2023".to_string(),
        timezone: "Asia/Shanghai".to_string(),
    };

    let result = WakaTimeDao::insert_range(
        &range.id,
        &range.date,
        &range.start,
        &range.end,
        &range.text,
        &range.timezone,
    )
    .await;

    let res = match result {
        Ok(r) => r.to_string(),
        Err(err) => {
            format!("{}", err);
            err.to_string()
        }
    };
    res
}
