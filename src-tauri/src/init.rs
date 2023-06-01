use crate::{db::db_conn, tools::DB, utils::FileUtils};
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

        if exists {
            println!("数据库已存在");
            return Ok(());
        }

        println!("正在创建数据库");
        let schema = Schema::new(DbBackend::Sqlite);
        let tables = vec![
            schema.create_table_from_entity(range::Entity),
            schema.create_table_from_entity(categories::Entity),
            schema.create_table_from_entity(dependencies::Entity),
            schema.create_table_from_entity(editors::Entity),
            schema.create_table_from_entity(projects::Entity),
            schema.create_table_from_entity(grand_total::Entity),
            schema.create_table_from_entity(languages::Entity),
            schema.create_table_from_entity(machines::Entity),
            schema.create_table_from_entity(operating_systems::Entity),
        ];

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let db = rt.block_on(DB.get_or_init(|| db_conn()));
        let builder = db.get_database_backend();

        for table in tables {
            rt.block_on(db.execute(builder.build(&table)))?;
        }

        Ok(())
    }
}
