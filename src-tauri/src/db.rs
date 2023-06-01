use std::time::Duration;

use crate::tools::DB_PATH;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(DB_PATH.to_owned());

    opt.max_connections(10)
        .min_connections(1)
        .sqlx_logging(true)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    db
}
