#[warn(dead_code)]
pub fn date_to_num(date: &str) -> String {
    date.replace("-", "")
}