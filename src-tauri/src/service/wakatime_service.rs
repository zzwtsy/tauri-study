use crate::dao::WakaTimeDao;
use entity::wakatime::{WakaTimeActiveModel, WakaTimeJsonVec};
use tokio::try_join;
pub struct WakaTimeService {}

impl WakaTimeService {
    pub async fn add_wakatime_data(wakatime_active_model: WakaTimeJsonVec) -> anyhow::Result<()> {
        for active_model in wakatime_active_model {
            let active_model: WakaTimeActiveModel = active_model.into();

            WakaTimeDao::insert_range(active_model.range).await?;
            let dependencies_insert = WakaTimeDao::insert_dependencies(active_model.dependencies);
            let editors_insert = WakaTimeDao::insert_editors(active_model.editors);
            let projects_insert = WakaTimeDao::insert_projects(active_model.projects);
            let grand_total_insert = WakaTimeDao::insert_grand_total(active_model.grand_total);
            let languages_insert = WakaTimeDao::insert_languages(active_model.languages);
            let machines_insert = WakaTimeDao::insert_machines(active_model.machines);
            let operating_systems_insert =
                WakaTimeDao::insert_operating_systems(active_model.operating_systems);
            let categories_insert = WakaTimeDao::insert_categories(active_model.categories);

            try_join!(
                dependencies_insert,
                editors_insert,
                projects_insert,
                grand_total_insert,
                languages_insert,
                machines_insert,
                operating_systems_insert,
                categories_insert
            )?;
        }

        Ok(())
    }

    // pub async fn add_wakatime_data(wakatime_active_model: WakaTimeJsonVec) -> anyhow::Result<()> {
    //     for active_model in wakatime_active_model {
    //         let active_model: WakaTimeActiveModel = active_model.into();

    //         WakaTimeDao::insert_range(active_model.range).await?;
    //         WakaTimeDao::insert_dependencies(active_model.dependencies).await?;
    //         WakaTimeDao::insert_editors(active_model.editors).await?;
    //         WakaTimeDao::insert_projects(active_model.projects).await?;
    //         WakaTimeDao::insert_grand_total(active_model.grand_total).await?;
    //         WakaTimeDao::insert_languages(active_model.languages).await?;
    //         WakaTimeDao::insert_machines(active_model.machines).await?;
    //         WakaTimeDao::insert_operating_systems(active_model.operating_systems).await?;
    //         WakaTimeDao::insert_categories(active_model.categories).await?;
    //     }

    //     Ok(())
    // }
}
