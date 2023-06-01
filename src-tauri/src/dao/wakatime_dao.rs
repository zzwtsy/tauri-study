use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, TransactionTrait};

use crate::db::db_conn;
use crate::tools::DB;
use entity::{
    categories, dependencies, editors, grand_total, languages, machines, operating_systems,
    prelude::{
        Categories, Dependencies, Editors, GrandTotal, Languages, Machines, OperatingSystems,
        Projects, Range,
    },
    projects, range,
};

/// DAO 结构体，用于操作数据库。
pub struct WakaTimeDao {}

impl WakaTimeDao {
    async fn insert_entities<E, M>(models: Vec<M>) -> anyhow::Result<()>
    where
        E: EntityTrait + Clone + 'static + Send,
        M: ActiveModelTrait<Entity = E> + Clone + 'static + Send,
    {
        let db = DB.get_or_init(|| db_conn()).await;
        let entities = E::insert_many(models);

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                entities.exec(txn).await?;
                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn insert_categories(
        categories_models: Vec<categories::ActiveModel>,
    ) -> anyhow::Result<()> {
        if categories_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Categories, _>(categories_models).await?;

        Ok(())
    }

    pub async fn insert_dependencies(
        dependencies_models: Vec<dependencies::ActiveModel>,
    ) -> anyhow::Result<()> {
        if dependencies_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Dependencies, _>(dependencies_models).await?;

        Ok(())
    }

    pub async fn insert_editors(editors_models: Vec<editors::ActiveModel>) -> anyhow::Result<()> {
        if editors_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Editors, _>(editors_models).await?;

        Ok(())
    }

    pub async fn insert_grand_total(
        grand_total_model: grand_total::ActiveModel,
    ) -> anyhow::Result<()> {
        Self::insert_entities::<GrandTotal, _>(vec![grand_total_model]).await?;

        Ok(())
    }

    pub async fn insert_languages(
        languages_models: Vec<languages::ActiveModel>,
    ) -> anyhow::Result<()> {
        if languages_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Languages, _>(languages_models).await?;

        Ok(())
    }

    pub async fn insert_machines(
        machines_models: Vec<machines::ActiveModel>,
    ) -> anyhow::Result<()> {
        if machines_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Machines, _>(machines_models).await?;

        Ok(())
    }

    pub async fn insert_operating_systems(
        operating_systems_models: Vec<operating_systems::ActiveModel>,
    ) -> anyhow::Result<()> {
        if operating_systems_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<OperatingSystems, _>(operating_systems_models).await?;

        Ok(())
    }

    pub async fn insert_projects(
        projects_models: Vec<projects::ActiveModel>,
    ) -> anyhow::Result<()> {
        if projects_models.is_empty() {
            return Ok(());
        }

        Self::insert_entities::<Projects, _>(projects_models).await?;

        Ok(())
    }

    pub async fn insert_range(range_model: range::ActiveModel) -> anyhow::Result<()> {
        Self::insert_entities::<Range, _>(vec![range_model]).await?;

        Ok(())
    }
}
