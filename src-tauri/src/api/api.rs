use entity::wakatime::WakaTimeModelVec;

use crate::res::GithubGist;
use crate::utils::HttpUtils;

pub async fn get_gist_raw_url(gist_id: String) -> anyhow::Result<Vec<String>> {
    let url = format!("https://api.github.com/gists/{}", gist_id);
    let raw_url = HttpUtils::get_json::<GithubGist>(&url, None).await?;

    let raw_url = raw_url
        .files_map
        .values()
        .filter(|v| v.filename.starts_with("summaries"))
        .map(|v| v.raw_url.clone())
        .collect::<Vec<String>>();

    Ok(raw_url)
}

pub async fn get_posts(raw_urls: Vec<String>) -> anyhow::Result<Vec<WakaTimeModelVec>> {
    let response_futures = raw_urls
        .iter()
        .map(|url| HttpUtils::get_json::<WakaTimeModelVec>(url, None));

    let wakatime_model_vec = futures::future::try_join_all(response_futures).await?;

    Ok(wakatime_model_vec)
}
