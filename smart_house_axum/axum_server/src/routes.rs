use crate::{AppState, HouseWrapperState};
use axum::extract::State;
use axum::{
    http::{
        header::{HeaderMap, ACCEPT},
        status::StatusCode,
    },
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use lib_shouse::home::home::home::*;
use minijinja::{context, Environment};
use serde::Deserialize;
use serde::Serialize;
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
pub async fn returns_json() -> axum::response::Response {
    let hello = Hello {
        name: String::from("world"),
    };

    Json(hello).into_response()
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
