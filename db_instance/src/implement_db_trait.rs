#[allow(unused_imports)]
pub mod implement {
    use crate::DbQueries;
    use crate::ErrorDb;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use console::{style, Term};
    use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
    use std::fmt::Write;
    use std::thread;
    use std::time::Duration;
    use thiserror::Error;
    use tracing::{Instrument, Level};
    use tracing_subscriber;
    use tracing_subscriber::fmt;
    #[derive(Clone, FromRow, Debug)]
    pub struct RoomGeneric {
        roomid: i64,
        roomname: String,
        info: String,
        attached_to_house: String,
    }
    #[derive(Clone, FromRow, Debug)]
    pub struct SHouseGeneric {
        id: i64,
        housename: String,
        active: bool,
    }

    #[async_trait]
    impl DbQueries for &SqlitePool {
        async fn add_house(self, housename: &str) -> Result<(), ErrorDb> {
            tracing::info!("inserting house with name: {}", housename);
            let query_str = format!("INSERT INTO smarthouse (housename) VALUES (?)");
            match sqlx::query(&query_str).bind(housename).execute(self).await {
                Ok(_r) => {
                    tracing::info!("added new house with name {}", housename);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("error:{e} while inserting house with name {}", housename);
                    Err(ErrorDb::ErrorQuery("error inserting house".to_owned()))
                }
            }
        }

        async fn activate_house(self, housename: &str, val: bool) -> Result<(), ErrorDb> {
            match sqlx::query("UPDATE smarthouse SET active=? WHERE housename=? ")
                .bind(val)
                .bind(housename)
                .execute(self)
                .await
            {
                Ok(_r) => {
                    tracing::info!("state of house {} is {}", housename, val);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("error:{e} while activating house with name {}", housename);
                    Err(ErrorDb::ErrorQuery("error activate house".to_owned()))
                }
            }
        }

        async fn add_room(self, roomname: &str, info: &str) -> Result<(), ErrorDb> {
            let query_str = format!("INSERT INTO rooms (roomname,info) VALUES (?,?)");
            match sqlx::query(&query_str)
                .bind(roomname)
                .bind(info)
                .execute(self)
                .await
            {
                Ok(_r) => {
                    tracing::info!("adding room {}", roomname);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("error:{e} while adding room {}", roomname);
                    Err(ErrorDb::ErrorQuery("error adding room".to_owned()))
                }
            }
        }
        async fn info_about_room(self, roomname: &str) -> Result<String, ErrorDb> {
            match sqlx::query_as::<_, RoomGeneric>(
                "SELECT roomid, roomname, info, attached_to_house FROM rooms WHERE roomname=?",
            )
            .bind(roomname)
            .fetch_one(self)
            .await
            {
                Ok(_results) => {
                    let out = format!(
                        "id: [{}],roomname:{},info: {}, assigned to house: {} ",
                        _results.roomid,
                        &_results.roomname,
                        &_results.info,
                        &_results.attached_to_house
                    );

                    println!("{}", style(&out).green());
                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info about room {}", roomname);
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about room".to_owned(),
                    ))
                }
            }
        }

        async fn assign_room_to_house(
            self,
            housename: &str,
            roomname: &str,
        ) -> Result<(), ErrorDb> {
            match sqlx::query("UPDATE rooms SET attached_to_house=? WHERE roomname=? ")
                .bind(housename)
                .bind(roomname)
                .execute(self)
                .await
            {
                Ok(_r) => {
                    tracing::info!("assigning room {} to house {}", roomname, housename);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("error:{e} while assigning room {}", roomname);
                    Err(ErrorDb::ErrorQuery("error assigning room".to_owned()))
                }
            }
        }

        async fn info_about_all_rooms(self) -> Result<String, ErrorDb> {
            match sqlx::query_as::<_, RoomGeneric>(
                "SELECT roomid, roomname, info, attached_to_house FROM rooms",
            )
            .fetch_all(self)
            .await
            {
                Ok(_results) => {
                    let mut out = String::from("---info about all rooms---\n");
                    for room in _results {
                        writeln!(
                            out,
                            "id: [{}] roomname: {} , info {}, attached to house: {}",
                            room.roomid, &room.roomname, &room.info, &room.attached_to_house,
                        )
                        .expect("error while writing to string");
                    }
                    println!("{}", style(&out).green());

                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info all rooms");
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about all rooms".to_owned(),
                    ))
                }
            }
        }

        async fn info_about_house(self, house: &str) -> Result<String, ErrorDb> {
            match sqlx::query_as::<_, SHouseGeneric>(
                "SELECT id, housename, active FROM smarthouse WHERE housename=?",
            )
            .bind(house)
            .fetch_one(self)
            .await
            {
                Ok(_results) => {
                    let out = format!(
                        "id: [{}], housename:{}, active: {}",
                        _results.id, &_results.housename, _results.active,
                    );

                    println!("{}", style(&out).cyan());
                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info about house {}", house);
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about house".to_owned(),
                    ))
                }
            }
        }
    }
}
