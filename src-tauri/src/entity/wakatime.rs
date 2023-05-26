use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type WakaTimeRoot = Vec<WakaTime>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct WakaTime {
    pub categories: Vec<Category>,
    pub dependencies: Vec<Dependency>,
    pub editors: Vec<Editor>,
    pub grand_total: GrandTotal,
    pub languages: Vec<Language>,
    pub machines: Vec<Machine>,
    pub operating_systems: Vec<OperatingSystem>,
    pub projects: Vec<Project>,
    pub range: Range,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: i64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Dependency {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: f64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Editor {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: i64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct GrandTotal {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Language {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: f64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Machine {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub machine_name_id: String,
    pub minutes: i64,
    pub name: String,
    pub percent: i64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct OperatingSystem {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: i64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub name: String,
    pub percent: f64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Range {
    pub id: i32,
    pub date: String,
    pub end: String,
    pub start: String,
    pub text: String,
    pub timezone: String,
}