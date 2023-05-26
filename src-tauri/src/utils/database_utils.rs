use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub struct DatabaseUtils {
}

impl DatabaseUtils {
    pub async fn new(db_path: &str) -> anyhow::Result<SqlitePool> {
        // 创建数据库连接池。
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_path).await?;

        Ok(db_pool)
    }
}