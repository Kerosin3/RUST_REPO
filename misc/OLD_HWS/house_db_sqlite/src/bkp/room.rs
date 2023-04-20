#[allow(unused_imports)]
pub mod rooms_lib {
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
    pub struct RoomGeneric {
        roomid: i64,
        roomname: String,
        info: String,
        attached_to_house: String,
    }
    impl RoomGeneric {
        pub async fn insert_room(
            db: &SqlitePool,
            col_name: &str,
            item: &str,
        ) -> anyhow::Result<()> {
            tracing::info!("inserting data to {}, item: {}", col_name, item);
            let query_str = format!("INSERT INTO rooms ({col_name}) VALUES (?)");
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
        pub async fn assign_house(
            db: &SqlitePool,
            roomname: &str,
            housename: &str,
        ) -> anyhow::Result<()> {
            match sqlx::query("UPDATE rooms SET attached_to_house=? WHERE roomname=? ")
                .bind(housename)
                .bind(roomname)
                .execute(db)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!(ErrorDb::ErrorQuery(e.to_string())),
            }
        }

        pub async fn fetch_sh_info_columns(db: &SqlitePool) -> anyhow::Result<()> {
            match sqlx::query_as::<_, RoomGeneric>(
                "SELECT roomid, roomname, info, attached_to_house FROM rooms",
            )
            .fetch_all(db)
            .await
            {
                Ok(_results) => {
                    for room in _results {
                        println!(
                            "[{}] name: {} ,info {} house: {}",
                            room.roomid, &room.roomname, &room.info, &room.attached_to_house
                        );
                    }
                    Ok(())
                }
                Err(e) => {
                    anyhow::bail!(ErrorDb::ErrorQuery(e.to_string()))
                }
            }
        }
    }
}
