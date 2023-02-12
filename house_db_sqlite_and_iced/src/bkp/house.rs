#[allow(unused_imports)]
pub mod house_lib {
    use crate::ErrorDb;
    use anyhow::anyhow;
    use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
    use std::thread;
    use std::time::Duration;
    use thiserror::Error;
    use tracing::Level;
    use tracing_subscriber;
    use tracing_subscriber::fmt;
    #[derive(Clone, FromRow, Debug)]
    pub struct SHouseGeneric {
        id: i64,
        housename: String,
        active: bool,
    }

    impl SHouseGeneric {
        pub async fn insert_house(
            db: &SqlitePool,
            col_name: &str,
            item: &str,
        ) -> anyhow::Result<()> {
            tracing::info!("inserting data to {}, item: {}", col_name, item);
            let query_str = format!("INSERT INTO smarthouse ({col_name}) VALUES (?)");
            match sqlx::query(&query_str).bind(item).execute(db).await {
                Ok(_r) => {
                    println!("Query result: {:?}", _r);
                    Ok(())
                }
                Err(e) => {
                    println!("error is {}", e);
                    anyhow::bail!(ErrorDb::ErrorQuery(e.to_string()))
                }
            }
        }
        pub async fn activate<'a, T>(
            db: &SqlitePool,
            where_item: &'a str,
            value: T,
        ) -> anyhow::Result<()>
        where
            T: sqlx::Encode<'a, Sqlite> + Send + Sync + sqlx::Type<Sqlite> + 'a,
        {
            match sqlx::query("UPDATE smarthouse SET active=? WHERE housename=? ")
                .bind(value)
                .bind(where_item)
                .execute(db)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!(ErrorDb::ErrorQuery(e.to_string())),
            }
        }
        pub async fn add_room(
            db: &SqlitePool,
            where_item: &str,
            value: &str,
        ) -> anyhow::Result<()> {
            match sqlx::query("UPDATE smarthouse SET aroom=? WHERE housename=? ")
                .bind(value)
                .bind(where_item)
                .execute(db)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!(ErrorDb::ErrorQuery(e.to_string())),
            }
        }

        pub async fn fetch_sh_info_columns(db: &SqlitePool) -> anyhow::Result<()> {
            match sqlx::query_as::<_, SHouseGeneric>("SELECT id, housename, active FROM smarthouse")
                .fetch_all(db)
                .await
            {
                Ok(_results) => {
                    for house in _results {
                        println!(
                            "[{}] name: {}, active: {}",
                            house.id, &house.housename, house.active
                        );
                    }
                    println!("OK!");
                    Ok(())
                }
                Err(e) => {
                    anyhow::bail!(ErrorDb::ErrorQuery(e.to_string()))
                }
            }
        }
    }
}
