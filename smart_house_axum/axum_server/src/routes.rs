use crate::{AppState, HouseWrapperState};
use axum::extract::State;
use axum::{extract::Query, routing::get, Router};
use axum::{
    http::{
        header::{HeaderMap, ACCEPT},
        status::StatusCode,
    },
    response::{IntoResponse, Json, Response},
    routing::post,
};
use lib_shouse::home::home::home::*;
use minijinja::{context, Environment};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::sync::MutexGuard;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
/*
pub async fn devices_main_page(State(state): State<AppState>) -> axum::response::Html<String> {
    //include_str!("./pages/test1.html").into()
    let mut env = Environment::new();
    env.add_template("devicesMP.txt", include_str!("./pages/devices.txt"))
        .unwrap();
    let tmpl = env.get_template("devicesMP.txt").unwrap();
    tracing::info!("sending rendered template");
    tmpl.render(context!(names => state.0 )).unwrap().into()
}*/
pub async fn devices_main_page(
    State(state): State<HouseWrapperState>,
) -> axum::response::Html<String> {
    //include_str!("./pages/test1.html").into()
    let x: MutexGuard<SmartHouse> = state.0.lock().unwrap();
    println!("rooms: {:?}", x.get_all_rooms().unwrap());
    //x.test_whether_a_dev_exists("smart_socket_#0")
    //    );
    let mut env = Environment::new();
    env.add_template("devicesMP.txt", include_str!("./pages/devices.txt"))
        .unwrap();
    let tmpl = env.get_template("devicesMP.txt").unwrap();
    tracing::info!("sending rendered template");
    tmpl.render(context!(names => ["adasd","dasdadd","aaaaaaa"] ))
        .unwrap()
        .into()
}

#[derive(Serialize)]
struct Hello {
    name: String,
}
//response with JSON
pub async fn returns_json() -> impl IntoResponse {
    let hello = Hello {
        name: String::from("world"),
    };

    //    Json(hello).into_response()
    (StatusCode::CREATED, Json(hello))
}

//http  GET 127.0.0.1:8080/json1 Accept:application/json -v
//http  GET 127.0.0.1:8080/json1 Accept:text/html -v
pub async fn heterogeneous_handle(headers: HeaderMap) -> Response {
    match headers.get(ACCEPT).map(|x| x.as_bytes()) {
        Some(r) if find_subsequence(r, b"text/html").is_some() => {
            String::from("Hello, world!").into_response()
        }
        Some(r) if find_subsequence(r, b"application/json").is_some() => {
            let hello = Hello {
                name: String::from("world"),
            };
            Json(hello).into_response()
        }
        _ => StatusCode::BAD_REQUEST.into_response(),
    }
}
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
//https://github.com/tokio-rs/axum/blob/main/examples/parse-body-based-on-content-type/src/main.rs
//
//
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    email: String,
    password: String,
}
//http POST 127.0.0.1:8080/json2 email="dasdasd" password="123244" -v
pub async fn create_user(Json(payload): Json<CreateUser>) {
    dbg!("{:?}", payload);
}
//http POST 127.0.0.1:8080/getdevproperty devname="termometer_#0"  -v
pub async fn get_property(
    State(home_obj): State<HouseWrapperState>,
    Json(payload): Json<DeviceG>,
) -> impl IntoResponse {
    if home_obj.0.is_poisoned() {
        tracing::error!("poisoned mutex meet");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeviceState {
                devname: payload.devname,
                info: "error while processing request".to_string(),
            }),
        );
    }
    if let Ok(guard) = home_obj.0.try_lock() {
        if let Some(info_r_d) = guard.test_whether_a_dev_exists(&payload.devname) {
            // room device
            if let Ok(info_prop) = guard.get_device_property(info_r_d.1.as_str()) {
                tracing::info!("sending info about device {}", payload.devname);
                (
                    StatusCode::OK,
                    Json(DeviceState {
                        devname: payload.devname,
                        info: info_prop,
                    }),
                )
            } else {
                tracing::error!("error while getting device property");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(DeviceState {
                        devname: payload.devname,
                        info: "cannot get device property".to_string(),
                    }),
                );
            }
        } else {
            tracing::error!("device: {} not found!", payload.devname);
            (
                StatusCode::NOT_FOUND,
                Json(DeviceState {
                    devname: payload.devname,
                    info: "device not found in the server!".to_string(),
                }),
            )
        }
    } else {
        tracing::error!("error locking mutex");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeviceState {
                devname: payload.devname,
                info: "error locking mutex".to_string(),
            }),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceG {
    devname: String,
}
#[derive(Serialize, Debug)]
pub struct DeviceState {
    devname: String,
    info: String,
}
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
//#[debug_handler]
pub async fn get_devices(
    State(home_obj): State<HouseWrapperState>,
    Json(payload): Json<DeviceG>,
) -> impl IntoResponse {
    //    dbg!("{:?}", &payload);
    if home_obj.0.is_poisoned() {
        tracing::error!("poisoned mutex meet");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeviceState {
                devname: payload.devname,
                info: "error while processing request".to_string(),
            }),
        );
    }
    if let Ok(guard) = home_obj.0.try_lock() {
        if let Some(info_r_d) = guard.test_whether_a_dev_exists(&payload.devname) {
            tracing::info!("device {} was requested", payload.devname);
            (
                StatusCode::OK,
                Json(DeviceState {
                    devname: info_r_d.1,
                    info: "device found!".to_string(),
                }),
            )
        } else {
            tracing::info!("device {} was not found in the server", payload.devname);
            (
                StatusCode::NOT_FOUND,
                Json(DeviceState {
                    devname: payload.devname,
                    info: "device not found in the server!".to_string(),
                }),
            )
        }
    } else {
        tracing::error!("error locking mutex");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeviceState {
                devname: payload.devname,
                info: "error locking mutex".to_string(),
            }),
        )
    }
}
