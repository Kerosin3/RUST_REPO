use crate::HouseWrapperState;
use axum::extract::State;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use axum::{
    http::{
        header::{HeaderMap, ACCEPT},
        status::StatusCode,
    },
    response::{IntoResponse, Json, Response},
    routing::post,
};
use lib_shouse::home::home::home::*;
use serde::{de, Deserialize, Deserializer, Serialize};
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Turning {
    devname: String,
    status: String,
}
impl Default for Turning {
    fn default() -> Self {
        Self {
            devname: "default_dev".to_string(),
            status: "default".to_string(),
        }
    }
}

//http --form GET 127.0.0.1:8080/device devname=="smart_socket_#0" status=="off"
pub async fn turning_the_device(
    State(home_obj): State<HouseWrapperState>,
    command: Option<Query<Turning>>,
) -> StatusCode {
    if let Some(Query(req_str)) = command {
        if let Ok(mut guard) = home_obj.0.try_lock() {
            if let Some((room, dev)) = guard.test_whether_a_dev_exists(&req_str.devname) {
                if req_str.status.contains("on") {
                    guard.change_dev_state_in_room(&room, &dev, true).unwrap();
                    tracing::info!("turning ON device {}", &dev);
                    StatusCode::OK
                } else if req_str.status.contains("off") {
                    tracing::info!("turning OFF device {}", &dev);
                    guard.change_dev_state_in_room(&room, &dev, false).unwrap();
                    StatusCode::OK
                } else {
                    tracing::info!("strange input command, turning OFF device {}", &dev);
                    guard.change_dev_state_in_room(&room, &dev, false).unwrap();
                    StatusCode::OK
                }
            } else {
                tracing::error!("no device {} exists", req_str.devname);
                StatusCode::NOT_FOUND
            }
        } else {
            tracing::error!("cannot lock mutex");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        tracing::error!("error processing query");
        StatusCode::INTERNAL_SERVER_ERROR
    }
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
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(DeviceState {
                        devname: payload.devname,
                        info: "cannot get device property".to_string(),
                    }),
                )
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
//for debug
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/* jinja template
pub async fn devices_main_page(State(state): State<AppState>) -> axum::response::Html<String> {
    //include_str!("./pages/test1.html").into()
    let mut env = Environment::new();
    env.add_template("devicesMP.txt", include_str!("./pages/devices.txt"))
        .unwrap();
    let tmpl = env.get_template("devicesMP.txt").unwrap();
    tracing::info!("sending rendered template");
    tmpl.render(context!(names => state.0 )).unwrap().into()
}*/
//http  GET 127.0.0.1:8080/json1 Accept:application/json -v
//http  GET 127.0.0.1:8080/json1 Accept:text/html -v
/*
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
*/
//https://github.com/tokio-rs/axum/blob/main/examples/parse-body-based-on-content-type/src/main.rs
//
