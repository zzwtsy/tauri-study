use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubGist {
    #[serde(rename = "files")]
    pub files_map: HashMap<String, GistFile>,
    pub owner: Owner
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GistFile {
    pub filename: String,
    pub raw_url: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub login: Option<String>,
}
