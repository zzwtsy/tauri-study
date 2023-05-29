use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, TransactionTrait};

use crate::{db::db_conn, tools::DB};
use entity::*;

/// DAO 结构体，用于操作数据库。
pub struct WakaTimeDao {}

#[warn(dead_code)]
impl WakaTimeDao {
    pub async fn insert_categories(
        categories_models: Vec<categories::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                categories::Entity::insert_many(categories_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_dependencies(
        dependencies_models: Vec<dependencies::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                dependencies::Entity::insert_many(dependencies_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_editors(editors_models: Vec<editors::ActiveModel>) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                editors::Entity::insert_many(editors_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_grand_total(
        grand_total_model: grand_total::ActiveModel,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                grand_total_model.insert(txn).await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_languages(
        languages_models: Vec<languages::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                languages::Entity::insert_many(languages_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_machines(
        machines_models: Vec<machines::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                machines::Entity::insert_many(machines_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_operating_systems(
        operating_systems_models: Vec<operating_systems::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                operating_systems::Entity::insert_many(operating_systems_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_projects(
        projects_models: Vec<projects::ActiveModel>,
    ) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                projects::Entity::insert_many(projects_models)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_range(range_model: range::ActiveModel) -> anyhow::Result<()> {
        let db = DB.get_or_init(|| db_conn()).await;

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                range_model.insert(txn).await?;

                Ok(())
            })
        })
        .await?;

        Ok(())
    }
}
