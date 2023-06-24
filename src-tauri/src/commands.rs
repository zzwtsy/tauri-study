use chrono::NaiveDate;

use crate::{api, dao::WakaTimeDao, res::WakaTimeRes, service::WakaTimeService};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_all_wakatime_data() -> Vec<WakaTimeRes> {
    let result = WakaTimeService::select_all_wakatime_data().await;

    if result.is_err() {
        println!("err");
        return Vec::new();
    }
    let tmp = result.unwrap().first().unwrap().clone();
    println!("{:#?}", tmp);
    return vec![tmp];
}

#[tauri::command]
pub async fn query_wakatime_by_date_range() {
    let start = NaiveDate::parse_from_str("2023-05-24", "%Y-%m-%d")
        .unwrap()
        .to_string();
    let end = NaiveDate::parse_from_str("2023-05-30", "%Y-%m-%d")
        .unwrap()
        .to_string();
    let res = WakaTimeDao::select_editers_by_date_range(start, end).await;

    match res {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{:#?}", err),
    }
}

#[tauri::command]
pub async fn gist_id(id: i32) -> String {
    println!("{}", id);

    let raw_urls = api::get_gist_raw_url("070bce177f3e0bc2b012be8ecfa166e4".to_owned())
        .await
        .unwrap();

    let posts = api::get_posts(raw_urls).await;

    if posts.is_err() {
        let err = posts.err().unwrap();
        println!("posts err = {:#?}", err);
        return err.to_string();
    }

    let posts = posts.unwrap();

    let status = WakaTimeService::add_wakatime_data(posts).await;

    if status.is_err() {
        return status.err().unwrap().to_string();
    }

    let res = WakaTimeService::select_all_wakatime_data().await;

    match res {
        Ok(_) => "success".to_owned(),
        Err(err) => err.to_string(),
    }
}
