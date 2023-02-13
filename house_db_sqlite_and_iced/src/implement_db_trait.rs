#[allow(unused_imports)]
#[allow(dead_code)]
pub mod implement {
    use crate::DbQueries;
    use crate::ErrorDb;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use console::{style, Term};
    use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
    use std::fmt::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    use thiserror::Error;
    use tracing::{instrument, Level};
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
    #[derive(Clone, FromRow, Debug)]
    pub struct RoomInHouse {
        id: i64,
        housename: String,
        roomname: String,
        info: String,
        attached_to_house: String,
    }
    #[derive(Clone, FromRow, Debug)]
    pub struct DevInRoom {
        pub devid: i64,
        pub devname: String,
        pub attached_to_room: String,
        pub attached_to_house: String,
        pub info: String,
        pub active: bool,
        pub timestamp: String,
    }

    #[derive(Clone, FromRow, Debug)]
    pub struct Existance {
        roomname: String,
    }
    #[derive(Clone, FromRow, Debug)]
    pub struct ExistanceHome {
        housename: String,
    }
    #[derive(Clone, FromRow, Debug)]
    pub struct ExistanceDev {
        devname: String,
    }
    #[derive(Clone, FromRow, Debug)]
    pub struct DevInfo {
        info: String,
    }

    #[async_trait]
    impl DbQueries for Arc<SqlitePool> {
        /*
        async fn add_house(self, housename: &str) -> Result<(), ErrorDb> {
            tracing::info!("inserting house with name: {}", housename);
            if self.clone().test_whether_house_exists(housename).await? {
                // test existence
                return Err(ErrorDb::HouseAlreadyExists(housename.to_owned()));
            }
            let query_str = "INSERT INTO smarthouse (housename) VALUES (?)".to_string();
            sqlx::query(&query_str)
                .bind(housename)
                .execute(&*self.clone())
                .await?;
            tracing::info!("added new house with name {}", housename);
            Ok(())
        }
        async fn activate_device(
            self,
            housename: &str,
            roomname: &str,
            devname: &str,
            value: bool,
        ) -> Result<String, ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_owned()));
            }
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_owned()));
            }
            if !self
                .test_whether_dev_exists_in_room(devname, roomname)
                .await?
            {
                return Err(ErrorDb::DeviceNotExists(
                    devname.to_owned(),
                    roomname.to_owned(),
                ));
            }
            sqlx::query("UPDATE devices SET active=? WHERE devname=? AND attached_to_room=? AND attached_to_house=?")
                .bind(value)
                .bind(devname)
                .bind(roomname)
                .bind(housename)
                .execute(&*self)
                .await?;
            let out = format!("state of device {devname} in room {roomname} is {value}");
            tracing::info!("{out}");
            Ok(out)
        }
        async fn change_device_info(
            self,
            housename: &str,
            roomname: &str,
            devname: &str,
            info: String,
        ) -> Result<(), ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_owned()));
            }
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_owned()));
            }
            if !self
                .test_whether_dev_exists_in_room(devname, roomname)
                .await?
            {
                return Err(ErrorDb::DeviceNotExists(
                    devname.to_owned(),
                    roomname.to_owned(),
                ));
            }
            sqlx::query("UPDATE devices SET info=? WHERE devname=? AND attached_to_room=? AND attached_to_house=?")
                .bind(info)
                .bind(devname)
                .bind(roomname)
                .bind(housename)
                .execute(&*self)
                .await?;
            tracing::info!("changed device {} info", devname);
            Ok(())
        }
        */
        async fn get_device_info(
            &self,
            housename: &str,
            roomname: &str,
            devname: &str,
        ) -> Result<String, ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_owned()));
            }
            /*
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_owned()));
            }
            if !self
                .test_whether_dev_exists_in_room(devname, roomname)
                .await?
            {
                return Err(ErrorDb::DeviceNotExists(
                    devname.to_owned(),
                    roomname.to_owned(),
                ));
            }*/
            let result = sqlx::query_as::<_, DevInfo>(
                "SELECT info FROM devices WHERE attached_to_room=? AND attached_to_house=? AND devname=?",
            )
            .bind(roomname)
            .bind(housename)
            .bind(devname)
            .fetch_one(&*self.clone())
            .await?;

            Ok(result.info)
        }
        /*
                async fn test_whether_room_exists(self, roomname: &str) -> Result<bool, ErrorDb> {
                    match sqlx::query_as::<_, Existance>(
                        "SELECT roomname FROM rooms WHERE roomname=?",
                        //                "SELECT EXISTS(SELECT 1 FROM rooms WHERE roomname=? LIMIT 1)",/// ??????
                    )
                    .bind(roomname)
                    .fetch_one(&*self)
                    .await
                    {
                        Ok(_r) => {
                            tracing::info!("room {} EXISTS! ", roomname);
                            Ok(true)
                        }
                        Err(_e) => {
                            let err = format!("{_e:?}"); // get error
                            if err == *"RowNotFound" {
                                tracing::info!("room {} NOT EXISTS! ", roomname);
                                Ok(false)
                            } else {
                                tracing::error!("error while querring");
                                Err(ErrorDb::ErrorQuery("error testing room ".to_owned()))
                            }
                        }
                    }
                }
        */
        async fn test_whether_house_exists(&self, housename: &str) -> Result<bool, ErrorDb> {
            match sqlx::query_as::<_, ExistanceHome>(
                "SELECT housename FROM smarthouse WHERE housename=?",
            )
            .bind(housename)
            .fetch_one(&*self.clone())
            .await
            {
                Ok(_r) => {
                    tracing::info!("house {} EXISTS! ", _r.housename);
                    Ok(true)
                }
                Err(_e) => {
                    let err = format!("{_e:?}"); // get error
                    if err == *"RowNotFound" {
                        tracing::info!("house {} NOT EXISTS! ", housename);
                        Ok(false)
                    } else {
                        tracing::error!("error while querring {_e}");
                        Err(ErrorDb::ErrorQuery("error testing house".to_owned()))
                    }
                }
            }
        }
        /*
        async fn del_device(
            self,
            devname: &str,
            housename: &str, // not checked
            roomname: &str,  // checkd
        ) -> Result<(), ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_owned()));
            }
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_owned()));
            }
            if !self
                .test_whether_dev_exists_in_room(devname, roomname)
                .await?
            {
                return Err(ErrorDb::DeviceNotExists(
                    devname.to_owned(),
                    roomname.to_owned(),
                ));
            }
            sqlx::query("DELETE FROM devices WHERE devname=? AND attached_to_room=? AND attached_to_house=?")
                .bind(devname)
                .bind(roomname)
                .bind(housename)
                .execute(&*self)
                .await?;
            Ok(())
        }
        async fn add_device(
            self,
            devname: &str,
            housename: &str, // not checked
            roomname: &str,  // checkd
        ) -> Result<(), ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_owned()));
            }
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_owned()));
            }
            if self
                .test_whether_dev_exists_in_room(devname, roomname)
                .await?
            {
                return Err(ErrorDb::DeviceAlreadyExists(
                    devname.to_owned(),
                    roomname.to_owned(),
                ));
            }

            let query_str = "INSERT INTO devices (devname, info, active, attached_to_room, attached_to_house ) VALUES (?,?,?,?,?)".to_string();
            match sqlx::query(&query_str)
                .bind(devname)
                .bind("not initialized")
                .bind(false)
                .bind(roomname)
                .bind(housename)
                .execute(&*self)
                .await
            {
                Ok(_r) => {
                    tracing::info!(
                        "adding device:{} to room:{}, to house:{}",
                        devname,
                        roomname,
                        housename
                    );
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("error:{e} while adding device {}", devname);
                    Err(ErrorDb::ErrorQuery("error adding device".to_owned()))
                }
            }
        }

        async fn test_whether_dev_exists_in_room(
            self,
            devname: &str,
            roomname: &str,
        ) -> Result<bool, ErrorDb> {
            match sqlx::query_as::<_, ExistanceDev>(
                "SELECT devname FROM devices WHERE attached_to_room=? AND devname=?",
            )
            .bind(roomname)
            .bind(devname)
            .fetch_one(&*self)
            .await
            {
                Ok(_r) => {
                    tracing::info!("device {} EXISTS! in room {}", _r.devname, roomname);
                    Ok(true)
                }
                Err(_e) => {
                    let err = format!("{_e:?}"); // get error
                    if err == *"RowNotFound" {
                        tracing::info!("device {} NOT EXISTS! in room {}", devname, roomname);
                        Ok(false)
                    } else {
                        tracing::error!("error while querring {_e}");
                        Err(ErrorDb::ErrorQuery("error testing device ".to_owned()))
                    }
                }
            }
        }

        async fn get_all_devices_in_house(self, housename: &str) -> Result<String, ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_string()));
            }
            match sqlx::query_as::<_, DevInRoom>(
                "SELECT devid, devname, attached_to_house, attached_to_room, info, active, timestamp FROM devices"
            )
            .fetch_all(&*self)
            .await
            {
                Ok(_results) => {
                    let mut out = String::new();
                    writeln!(out, "---info about all devices in house {housename} ---")
                        .expect("error while writing to string");
                    for r in _results {
                        writeln!(
                            out,
                            "dev id [{}], dev name: {} room: {}, house: {} active:{} info: {}, data creation: {}",
                            r.devid, &r.devname, &r.attached_to_room, &r.attached_to_house, r.active, &r.info,&r.timestamp
                        )
                        .expect("error while writing to string");
                    }
                    println!("{}", style(&out).green());
                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info all devices in house");
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about all devices".to_owned(),
                    ))
                }
            }
        }
        async fn get_all_rooms_in_house(self, housename: &str) -> Result<String, ErrorDb> {
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_string()));
            }
            match sqlx::query_as::<_, RoomInHouse>(
                "SELECT smarthouse.id,
                smarthouse.housename,
                rooms.roomname,
                rooms.attached_to_house,
                rooms.info
                FROM smarthouse
                INNER JOIN rooms ON smarthouse.housename = rooms.attached_to_house",
            )
            .fetch_all(&*self)
            .await
            {
                Ok(_results) => {
                    let mut out = String::new();
                    writeln!(out, "---info about all rooms in house {housename} ---")
                        .expect("error while writing to string");
                    for r in _results {
                        writeln!(
                            out,
                            "house id [{}], house name: [{}] room: {}, info: {}",
                            r.id, r.housename, r.roomname, r.info
                        )
                        .expect("error while writing to string");
                    }
                    println!("{}", style(&out).green());
                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info all rooms in house");
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about all rooms".to_owned(),
                    ))
                }
            }
        }
        async fn activate_house(self, housename: &str, val: bool) -> Result<(), ErrorDb> {
            sqlx::query("UPDATE smarthouse SET active=? WHERE housename=? ")
                .bind(val)
                .bind(housename)
                .execute(&*self)
                .await?;
            tracing::info!("state of house {} is {}", housename, val);
            Ok(())
        }

        async fn add_room(self, roomname: &str, info: &str) -> Result<(), ErrorDb> {
            let query_str = "INSERT INTO rooms (roomname,info) VALUES (?,?)".to_string();
            sqlx::query(&query_str)
                .bind(roomname)
                .bind(info)
                .execute(&*self)
                .await?;

            tracing::info!("adding room {}", roomname);
            Ok(())
        }
        async fn info_about_room(self, roomname: &str) -> Result<String, ErrorDb> {
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(roomname.to_string()));
            }
            match sqlx::query_as::<_, RoomGeneric>(
                "SELECT roomid, roomname, info, attached_to_house FROM rooms WHERE roomname=?",
            )
            .bind(roomname)
            .fetch_one(&*self)
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
            if !self.test_whether_house_exists(housename).await? {
                return Err(ErrorDb::HouseNotExists(housename.to_string()));
            }
            if !self.test_whether_room_exists(roomname).await? {
                return Err(ErrorDb::RoomNotExists(housename.to_string()));
            }
            sqlx::query("UPDATE rooms SET attached_to_house=? WHERE roomname=? ")
                .bind(housename)
                .bind(roomname)
                .execute(&*self)
                .await?;
            tracing::info!("assigning room {} to house {}", roomname, housename);
            Ok(())
        }

        async fn info_about_all_rooms(self) -> Result<String, ErrorDb> {
            match sqlx::query_as::<_, RoomGeneric>(
                "SELECT roomid, roomname, info, attached_to_house FROM rooms",
            )
            .fetch_all(&*self)
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
            if !self.test_whether_house_exists(house).await? {
                return Err(ErrorDb::HouseNotExists(house.to_string()));
            }
            match sqlx::query_as::<_, SHouseGeneric>(
                "SELECT id, housename, active FROM smarthouse WHERE housename=?",
            )
            .bind(house)
            .fetch_one(&*self)
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
        */
    }
}
