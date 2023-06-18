use crate::{dao::WakaTimeDao, db::db_conn, res::*, tools::DB};
use chrono::NaiveDate;
use entity::{
    categories, dependencies, editors, grand_total, languages, machines, operating_systems,
    prelude::{
        Categories, Dependencies, Editors, GrandTotal, Languages, Machines, OperatingSystems,
        Projects, Range,
    },
    projects, range,
    wakatime::{WakaTimeActiveModel, WakaTimeModelVec},
};
use sea_orm::{EntityTrait, Iterable, QuerySelect};
use tokio::try_join;
pub struct WakaTimeService {}

macro_rules! select_without {
    ($table:ident, $entity:ident, $res_model:ident, $db:ident) => {
        $table::find()
            .select_only()
            .columns($entity::Column::iter().filter(|col| match col {
                $entity::Column::Id => false,
                $entity::Column::RangeId => false,
                _ => true,
            }))
            .into_model::<$res_model>()
            .all($db)
            .await?
    };
}

impl WakaTimeService {
    pub async fn query_wakatime_by_date_range(
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<ChartVo>> {
        let start = start.format("%Y-%m-%d").to_string();
        let end = end.format("%Y-%m-%d").to_string();

        if start > end {
            return Err(anyhow::anyhow!("开始时间不能大于结束时间"));
        }

        let models = WakaTimeDao::select_languages_by_date_range(start, end).await?;

        Ok(models)
    }

    pub async fn add_wakatime_data(
        wakatime_active_model: Vec<WakaTimeModelVec>,
    ) -> anyhow::Result<()> {
        for active_models in wakatime_active_model {
            for active_model in active_models {
                let active_model: WakaTimeActiveModel = active_model.into();
                println!("{:#?}", active_model);

                // 注意：range 表必须第一个插入数据，其它表中使用 range 表的 id 字段作为外键
                WakaTimeDao::insert_range(active_model.range).await?;

                try_join!(
                    WakaTimeDao::insert_dependencies(active_model.dependencies),
                    WakaTimeDao::insert_editors(active_model.editors),
                    WakaTimeDao::insert_projects(active_model.projects),
                    WakaTimeDao::insert_grand_total(active_model.grand_total),
                    WakaTimeDao::insert_languages(active_model.languages),
                    WakaTimeDao::insert_machines(active_model.machines),
                    WakaTimeDao::insert_operating_systems(active_model.operating_systems),
                    WakaTimeDao::insert_categories(active_model.categories),
                )?;
            }
        }

        Ok(())
    }

    pub async fn select_all_wakatime_data() -> anyhow::Result<Vec<WakaTimeRes>> {
        let db = DB.get_or_init(|| db_conn()).await;

        let mut wakatime: Vec<WakaTimeRes> = Vec::new();

        let range_models = Range::find()
            .select_only()
            .columns(range::Column::iter().filter(|col| match col {
                range::Column::Id => false,
                _ => true,
            }))
            .into_model()
            .all(db)
            .await?;

        for range_model in range_models {
            let categories = select_without!(Categories, categories, CategoryRes, db);

            let dependencies = select_without!(Dependencies, dependencies, DependencyRes, db);

            let editors = select_without!(Editors, editors, EditorRes, db);

            let projects = select_without!(Projects, projects, ProjectRes, db);

            let languages = select_without!(Languages, languages, LanguageRes, db);

            let machines = select_without!(Machines, machines, MachineRes, db);

            let operating_systems =
                select_without!(OperatingSystems, operating_systems, OperatingSystemRes, db);

            let grand_total = GrandTotal::find()
                .select_only()
                .columns(grand_total::Column::iter().filter(|col| match col {
                    grand_total::Column::Id => false,
                    grand_total::Column::RangeId => false,
                    _ => true,
                }))
                .into_model::<GrandTotalRes>()
                .one(db)
                .await?;

            let wakatime_vo = WakaTimeRes {
                categories,
                dependencies,
                editors,
                grand_total,
                languages,
                machines,
                operating_systems,
                projects,
                range: range_model,
            };

            wakatime.push(wakatime_vo);
        }

        Ok(wakatime)
    }
}
