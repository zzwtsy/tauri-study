use std::time::{SystemTime, UNIX_EPOCH};

/// 将日期转换为数字
///
/// # 参数
///
/// * `date` - 日期（yyyy-mm-dd格式）
///
/// # 返回值
///
/// * `i32` - 数字（yyyymmdd）
pub fn date_to_num(date: &str) -> i32 {
    let num: i32 = date.replace("-", "").parse().unwrap();
    num
}

pub fn get_timestamp() -> i64 {
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    timestamp
}
