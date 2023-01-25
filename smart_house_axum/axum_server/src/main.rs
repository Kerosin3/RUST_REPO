#![allow(dead_code)]
#![allow(unused_imports)]
use axum::{
    async_trait,
    body::StreamBody,
    extract::{FromRequest, Query, State},
    handler::Handler,
    http::{header::CONTENT_TYPE, Request, StatusCode, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, RequestExt, Router,
};
use axum_macros::debug_handler;
use lib_shouse::home::home::home::*;
use minijinja::{context, Environment};
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};
mod server_socket_struct;
mod server_termometer_struct;
use server_socket_struct::*;
use server_termometer_struct::*;
use std::sync::Mutex;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod routes;
use routes::*;

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq, Debug, Copy, Clone, Default))]
pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
}
#[derive(Clone)]
pub struct AppState(Vec<String>);
#[derive(Clone)]
pub struct HouseWrapperState(Arc<Mutex<SmartHouse>>);

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[tokio::main]
async fn main() {
    let subscriber = fmt()
        .compact()
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0 = wrap_device(SmartSocket::new());
    let dev1 = wrap_device(Termometer::new());
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev0 to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev1 to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    tracing::info!(
        "added device:{} to server",
        _dev0_handler.get_devname().unwrap()
    );
    tracing::info!(
        "added device:{} to server",
        _dev1_handler.get_devname().unwrap()
    );
    tracing::info!("start main server loop");
    let housestate = HouseWrapperState(Arc::new(Mutex::new(some_house)));

    //    let MainState = AppState(vec_of_strings!["alex", "peter", "alice"]);
    let app = Router::new()
        .fallback(fallback)
        .route("/devices", get(devices_main_page))
        .with_state(housestate)
        .route("/json0", get(returns_json))
        .route("/json2", post(create_user))
        .route("/json1", get(heterogeneous_handle));
    //.with_state(MainState);
    // Start the server. Note that for brevity, we do not add logging, graceful shutdown, etc.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn fallback(uri: axum::http::Uri) -> (StatusCode, String) {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

fn wrap_device<T: 'static + lib_shouse::home::home::home::Device + Send + Sync>(
    some_device: T,
) -> Arc<Mutex<dyn Device + Send>> {
    Arc::new(Mutex::new(some_device))
}
