#![feature(is_some_and)]
#![allow(unused_imports)]
#![allow(unused_results)]
#![allow(dead_code)]
use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
use std::thread;
use std::time::Duration;
use thiserror::Error;
use tracing::{instrument, Level};
use tracing_subscriber;
use tracing_subscriber::fmt;
mod implement_db_trait;
use console::{style, Term};
use futures::{future, prelude::*};
use implement_db_trait::implement::*;
use tokio::time;
//---------------------------------------------
//---------------------------------------------
const DB_URL: &str = "sqlite://sqlite.db";
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> anyhow::Result<()> {
    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("application started!");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let _crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    //--------------------------------------------------------------
    db.get_all_devices_in_house("smarthouse#1").await?;
    db.change_device_info(
        "smarthouse#1",
        "someroom#2",
        "device4",
        "updated info1".to_string(),
    )
    .await?;
    println!(
        "{}",
        db.get_device_info("smarthouse#1", "someroom#2", "device4")
            .await?
    );
    Ok(())
}
#[async_trait]
pub trait DbQueries: Send + Sync + std::fmt::Debug {
    async fn add_house(self, housename: &str) -> Result<(), ErrorDb>;
    async fn activate_house(self, housename: &str, val: bool) -> Result<(), ErrorDb>;
    async fn add_room(self, roomname: &str, info: &str) -> Result<(), ErrorDb>;
    async fn assign_room_to_house(self, housename: &str, roomname: &str) -> Result<(), ErrorDb>;
    async fn info_about_room(self, roomname: &str) -> Result<String, ErrorDb>;
    async fn info_about_house(self, house: &str) -> Result<String, ErrorDb>;
    async fn info_about_all_rooms(self) -> Result<String, ErrorDb>;
    async fn get_all_rooms_in_house(self, housename: &str) -> Result<String, ErrorDb>;
    async fn get_all_devices_in_house(self, housename: &str) -> Result<String, ErrorDb>;
    async fn test_whether_room_exists(self, roomname: &str) -> Result<bool, ErrorDb>;
    async fn test_whether_house_exists(self, housename: &str) -> Result<bool, ErrorDb>;
    async fn del_device(
        self,
        devname: &str,
        housename: &str,
        roomname: &str,
    ) -> Result<(), ErrorDb>;

    async fn test_whether_dev_exists_in_room(
        self,
        devname: &str,
        roomname: &str,
    ) -> Result<bool, ErrorDb>;
    async fn add_device(
        self,
        devname: &str,
        housename: &str,
        roomname: &str,
    ) -> Result<(), ErrorDb>;
    async fn activate_device(
        self,
        housename: &str,
        roomname: &str,
        devname: &str,
        value: bool,
    ) -> Result<String, ErrorDb>;
    async fn change_device_info(
        self,
        housename: &str,
        roomname: &str,
        devname: &str,
        info: String,
    ) -> Result<(), ErrorDb>;
    async fn get_device_info(
        self,
        housename: &str,
        roomname: &str,
        devname: &str,
    ) -> Result<String, ErrorDb>;
}
#[derive(Debug, Error)]
pub enum ErrorDb {
    #[error("error while executing query {0}")]
    ErrorQuery(String),
    #[error("not exists room name : {0}")]
    RoomNotExists(String),
    #[error("device exists : {0} in room: {1}")]
    DeviceAlreadyExists(String, String),
    #[error("house with name exists: {0}")]
    HouseAlreadyExists(String),
    #[error("house with name not exists: {0}")]
    HouseNotExists(String),
    #[error("device {0} not exists in room {1}")]
    DeviceNotExists(String, String),
}

impl From<sqlx::Error> for ErrorDb {
    fn from(value: sqlx::Error) -> Self {
        Self::ErrorQuery(value.to_string())
    }
}
