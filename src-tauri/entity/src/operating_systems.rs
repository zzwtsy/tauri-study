//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "operating_systems")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i64,
    #[serde(skip_deserializing, skip_serializing)]
    pub range_id: i64,
    pub decimal: String,
    pub digital: String,
    pub hours: i32,
    pub minutes: i32,
    pub name: String,
    #[sea_orm(column_type = "Double")]
    pub percent: f64,
    pub seconds: i32,
    pub text: String,
    #[sea_orm(column_type = "Double")]
    pub total_seconds: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::range::Entity",
        from = "Column::RangeId",
        to = "super::range::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Range,
}

impl Related<super::range::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Range.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
