use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromQueryResult)]
pub struct ChartVo {
    pub name: String,
    pub total_seconds: f64,
    pub date: String,
}
