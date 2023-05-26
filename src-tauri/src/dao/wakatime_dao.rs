use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};

use crate::tools::DB_PATH;

// DAO 结构体，用于操作数据库。
pub struct WakaTimeDao {}

static DB_POOL: Lazy<Pool<Sqlite>> = Lazy::new(|| {
    let pool = Pool::connect_lazy(DB_PATH).unwrap();

    pool
});

impl WakaTimeDao {
    pub async fn insert_grand_total(
        id: i32,
        range_id: i32,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;

        sqlx::query("INSERT INTO grand_total (id, date, timezone, total) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(range_id)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 time 表中插入数据。
    pub async fn insert_range(
        id: &i32,
        date: &str,
        start_date: &str,
        end_date: &str,
        text: &str,
        timezone: &str,
    ) -> anyhow::Result<u64> {
        let mut tx = DB_POOL.begin().await?;

        let result = sqlx::query(
            "INSERT INTO range (id, date, start, end, text, timezone) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(date)
        .bind(start_date)
        .bind(end_date)
        .bind(text)
        .bind(timezone)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(result.rows_affected())
    }

    // 向 category 表中插入数据。
    pub async fn insert_category(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO category (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 dependency 表中插入数据。
    pub async fn insert_dependency(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO dependency (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 editor 表中插入数据。
    pub async fn insert_editor(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO editor (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 language 表中插入数据。
    pub async fn insert_language(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO language (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 machine 表中插入数据。
    pub async fn insert_machine(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
        machine_name_id: &str,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO machine (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds, machine_name_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .bind(machine_name_id)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 operating_system 表中插入数据。
    pub async fn insert_operating_system(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO operating_system (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    // 向 project 表中插入数据。
    pub async fn insert_project(
        id: i32,
        range_id: i32,
        name: &str,
        decimal: f64,
        digital: &str,
        hours: i32,
        minutes: i32,
        seconds: i32,
        percent: f64,
        text: &str,
        total_seconds: f64,
    ) -> anyhow::Result<()> {
        let mut tx = DB_POOL.begin().await?;
        sqlx
        ::query(
            "INSERT INTO project (id, range_id, name, decimal, digital, hours, minutes, seconds, percent, text, total_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(id)
            .bind(range_id)
            .bind(name)
            .bind(decimal)
            .bind(digital)
            .bind(hours)
            .bind(minutes)
            .bind(seconds)
            .bind(percent)
            .bind(text)
            .bind(total_seconds)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
