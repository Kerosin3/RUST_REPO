use crate::{AppState, HouseWrapperState};
use axum::extract::State;
use lib_shouse::home::home::home::*;
use minijinja::{context, Environment};
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
