use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

pub const DB_PATH: &str = "sqlite:../data/database/wakatime.db?mode=rwc";
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
