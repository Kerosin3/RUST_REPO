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
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
mod server_socket_struct;
mod server_termometer_struct;
use server_socket_struct::*;
use server_termometer_struct::*;
use std::sync::Mutex;
use std::{net::SocketAddr, sync::Arc};
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
// main app state
#[derive(Clone)]
pub struct HouseWrapperState(Arc<Mutex<SmartHouse>>);
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
    tokio::spawn(async move { imitate_socket_power_change(_dev0_handler).await }); // change dev0
    tokio::spawn(async move { imitate_termo_data_achange(_dev1_handler).await }); // change dev1
    tracing::info!("start main server loop");
    let housestate = HouseWrapperState(Arc::new(Mutex::new(some_house)));

    let app = Router::new()
        .fallback(fallback)
        //        .route("/devices", get(devices_main_page))
        //       .with_state(housestate)
        .route("/json0", get(returns_json))
        .route("/json2", post(create_user))
        .route("/devices", post(get_devices))
        .with_state(housestate.clone())
        .route("/getdevproperty", post(get_property))
        .with_state(housestate.clone())
        .route("/json1", get(heterogeneous_handle));
    //.with_state(MainState);

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

async fn imitate_socket_power_change(handle: Device_Handler) {
    let mut rng: StdRng = SeedableRng::from_entropy();
    loop {
        handle
            .property_change_state(rng.gen_range(1000..5000))
            .unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}
async fn imitate_termo_data_achange(handle: Device_Handler) {
    let mut rng: StdRng = SeedableRng::from_entropy();
    loop {
        handle.property_change_state(rng.gen_range(30..90)).unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}
//debug
/*
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
*/
