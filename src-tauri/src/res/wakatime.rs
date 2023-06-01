use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WakaTimeRes {
    pub categories: Vec<CategoryRes>,
    pub dependencies: Vec<DependencyRes>,
    pub editors: Vec<EditorRes>,
    pub grand_total: Option<GrandTotalRes>,
    pub languages: Vec<LanguageRes>,
    pub machines: Vec<MachineRes>,
    pub operating_systems: Vec<OperatingSystemRes>,
    pub projects: Vec<ProjectRes>,
    pub range: RangeRes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct CategoryRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct DependencyRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct EditorRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct GrandTotalRes {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub minutes: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct LanguageRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct MachineRes {
    pub decimal: String,
    pub digital: String,
    pub hours: i64,
    pub machine_name_id: String,
    pub minutes: i64,
    pub name: String,
    pub percent: f64,
    pub seconds: i64,
    pub text: String,
    pub total_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct OperatingSystemRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct ProjectRes {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct RangeRes {
    pub date: String,
    pub end: String,
    pub start: String,
    pub text: String,
    pub timezone: String,
}
