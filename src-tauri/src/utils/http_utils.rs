use std::collections::HashMap;
use once_cell::sync::OnceCell;

use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, Url,
};
use serde::de::DeserializeOwned;


static INSTANCE: OnceCell<Client> = OnceCell::new();


/// 使用 GET 方法获取指定 URL 的 JSON 数据
///
/// # 参数
///
/// * `url`: 获取数据的 URL 地址
/// * `headers`: 请求头
///
/// # 返回
///
/// 返回一个 `Result<T, Error>` 对象，其中 `T` 是可反序列化类型的泛型参数，代表从 JSON 数据反序列化后的对象，`Error` 表示获取、解析 JSON 数据所发生的错误。
///
/// # 示例
///
/// ```rust
/// use std::error::Error;
/// use http_request::get;
///
/// #[derive(Debug, serde::Deserialize)]
/// struct Person {
///     id: u32,
///     name: String,
///     age: u32,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let url = "https://example.com/person/1";
///     let person: Person = get_json(&url).await?;
///
///     println!("{:?}", person);
///
///     Ok(())
/// }
/// ```
pub async fn get_json<T: DeserializeOwned>(
    url: &str,
    header: HashMap<HeaderName, HeaderValue>,
) -> anyhow::Result<T> {
    let url = Url::parse(url)?;

    let mut builder = INSTANCE.get().unwrap().get(url);

    if !header.is_empty() {
        for (key, val) in header {
            builder = builder.header(key, val);
        }
    }

    let response = builder
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(response)
}

/// 使用 POST 方法向指定 URL 提交 JSON 数据，并获取返回的 JSON 数据
///
/// # 参数
///
/// * `url`: 提交数据的 URL 地址
/// * `headers`: 请求头
/// * `body`: 要提交的 JSON 数据的字节数组
///
/// # 返回
///
/// 返回一个 `Result<T, Error>` 对象，其中 `T` 是可反序列化类型的泛型参数，代表从 JSON 数据反序列化后的对象，`Error` 表示获取、解析 JSON 数据所发生的错误。
///
/// # 示例
///
/// ```rust
/// use std::error::Error;
/// use http_request::post;
///
/// #[derive(Debug, serde::Deserialize)]
/// struct Result {
///     success: bool,
///     message: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let url = "https://example.com/api/v1/data";
///     let body = r#"{
///         "key": "value"
///     }"#.as_bytes();
///
///     let result: Result = post_json(&url, body).await?;
///
///     println!("{:?}", result);
///
///     Ok(())
/// }
/// ```
pub async fn post_json<T: DeserializeOwned>(
    url: &str,
    header: HashMap<HeaderName, HeaderValue>,
    body: &[u8],
) -> anyhow::Result<T> {
    let url = Url::parse(url)?;

    let mut builder = INSTANCE.get().unwrap().clone().post(url);

    if !header.is_empty() {
        for (key, val) in header {
            builder = builder.header(key, val);
        }
    }

    let response = builder
        .body(body.to_vec())
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(response)
}
