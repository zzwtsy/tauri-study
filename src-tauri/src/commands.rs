use crate::{api, service::WakaTimeService};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
