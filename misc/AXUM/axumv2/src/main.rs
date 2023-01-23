use axum::response::Html;
use axum::{
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
#[tokio::main]
pub async fn main() {
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/demo", get(get_demo_html))
        .route("/hello", get(hello_html));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("./pages/test1.html").into()
}
/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
// handle no route
pub async fn fallback(uri: axum::http::Uri) -> (StatusCode, String) {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}
