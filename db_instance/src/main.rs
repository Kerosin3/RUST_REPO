#[allow(unused_imports)]
#[allow(unused_results)]
use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
use std::thread;
use std::time::Duration;
use thiserror::Error;
use tracing::{instrument, Instrument, Level};
use tracing_subscriber;
use tracing_subscriber::fmt;
//mod house;
//use house::house_lib::*;
//mod room;
//use room::rooms_lib::*;
mod implement_db_trait;
use implement_db_trait::implement::*;
//---------------------------------------------
//---------------------------------------------
use console::{style, Term};
const DB_URL: &str = "sqlite://sqlite.db";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(Level::INFO)
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
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
    let main_house = "smarthouse#1".to_string();
    db.add_house(&main_house).await;
    db.activate_house(&main_house, true).await;
    //----------------------------
    let (room0, room1) = ("someroom#0".to_string(), "someroom#1".to_string());
    db.add_room(&room0, "hehe").await;
    db.add_room(&room1, "haha").await;
    db.assign_room_to_house(&main_house, &room0).await;

    println!("{}", style("getting info about rooms").yellow());
    db.info_about_room(&room0).await;
    db.info_about_all_rooms().await;
    db.info_about_house(&main_house).await;
    /*println!("{}", style("getting info about home").yellow());
    }*/
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
}
#[derive(Debug, Error)]
pub enum ErrorDb {
    #[error("error while executing query {0}")]
    ErrorQuery(String),
}
