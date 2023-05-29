use crate::{
    db::db_conn,
    tools::{DB, DB_PATH},
    utils::FileUtils,
};
use anyhow::Ok;
use entity::{
    categories, dependencies, editors, grand_total, languages, machines, operating_systems,
    projects, range,
};
use sea_orm::{ConnectionTrait, DbBackend, Schema};

pub struct Init {}

// 初始化相关代码
impl Init {
    pub fn create_table() -> anyhow::Result<()> {
        let exists = FileUtils::file_exists("../data/database/wakatime.db");
        println!("{:?}", exists);
        if exists {
            println!("数据库已存在");
            return Ok(());
        }
        println!("正在创建数据库");
        let schema = Schema::new(DbBackend::Sqlite);

        let range_stmt = schema.create_table_from_entity(range::Entity);
        let categories_stmt = schema.create_table_from_entity(categories::Entity);
        let dependencies_stmt = schema.create_table_from_entity(dependencies::Entity);
        let editors_stmt = schema.create_table_from_entity(editors::Entity);
        let projects_stmt = schema.create_table_from_entity(projects::Entity);
        let grand_total_stmt = schema.create_table_from_entity(grand_total::Entity);
        let languages_stmt = schema.create_table_from_entity(languages::Entity);
        let machines_stmt = schema.create_table_from_entity(machines::Entity);
        let operating_systems_stmt = schema.create_table_from_entity(operating_systems::Entity);

        let res = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let db = DB.get_or_init(|| db_conn()).await;

                let builder = db.get_database_backend();

                db.execute(builder.build(&range_stmt)).await?;
                db.execute(builder.build(&categories_stmt)).await?;
                db.execute(builder.build(&dependencies_stmt)).await?;
                db.execute(builder.build(&editors_stmt)).await?;
                db.execute(builder.build(&projects_stmt)).await?;
                db.execute(builder.build(&grand_total_stmt)).await?;
                db.execute(builder.build(&languages_stmt)).await?;
                db.execute(builder.build(&machines_stmt)).await?;
                db.execute(builder.build(&operating_systems_stmt)).await?;

                Ok(())
            });

        res
    }
}
