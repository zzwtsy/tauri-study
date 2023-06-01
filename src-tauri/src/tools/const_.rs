use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

// mode=rwc: 没有数据库文件时自动创建文件
pub const DB_PATH: &str = "sqlite:../data/database/wakatime.db?mode=rwc";
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
