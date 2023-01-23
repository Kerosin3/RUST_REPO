use axum::handler::Handler;
use axum::routing::get;
use axum::{extract::Path, response::Html, Router};
use minijinja::render;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(mainpage))
        .route("/exam", get(example))
        .route("/:profile_name", get(get_profile));
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<&'static str> {
    Html("hello world")
}

#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Profile {
    full_name: String,
    items: Vec<Items>,
}

async fn mainpage() -> Html<String> {
    let r = render!(HTML5EXAMPLE);
    Html(r)
}
async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    let orders_example = vec![
        Items {
            id: 1,
            name: "Article banana".into(),
        },
        Items {
            id: 2,
            name: "Article apple".into(),
        },
    ];
    let profile_example = Profile {
        full_name: profile_name,
        items: orders_example,
    };
    let r = render!(PROFILE_TEMPLATE, profile => profile_example );
    Html(r)
}

async fn example() -> Html<String> {
    Html(render!(PROFILE_TEMPLATE_2))
}

const PROFILE_TEMPLATE_2: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Smart house Main Page</title>
  <meta name="description" content="testim http server">
  <meta name="author" content="Alex V">

  <meta property="og:title" content="Smart house Main Page">
  <meta property="og:type" content="website">

  <link rel="icon" href="/favicon.ico">
  <link rel="icon" href="/favicon.svg" type="image/svg+xml">
  <link rel="apple-touch-icon" href="/apple-touch-icon.png">

  <link rel="stylesheet" href="css/styles.css?v=1.0">

</head>

<body>
    <p>Test Jinja</p>
  <script src="js/scripts.js"></script>
</body>
</html>
"#;
const PROFILE_TEMPLATE: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>A Basic HTML5 Template</title>
  <meta name="description" content="A basic HTML5 Template for new projects.">
  <meta name="author" content="Woile">
</head>

<body>
    <h1>Profile of {{ profile.full_name|title }}</h1>
    <p>This is a template example to show some functionality</p>
    <h2>Items</h3>
    <ul>
        {% for item in profile.items %}
        <li>{{ item.name }} ({{ item.id }})</li>
        {% endfor %}
    <ul>
</body>
</html>
"#;

const HTML5EXAMPLE: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>A Basic HTML5 Template</title>
  <meta name="description" content="A simple HTML5 Template for new projects.">
  <meta name="author" content="SitePoint">

  <meta property="og:title" content="A Basic HTML5 Template">
  <meta property="og:type" content="website">
  <meta property="og:url" content="https://www.sitepoint.com/a-basic-html5-template/">
  <meta property="og:description" content="A simple HTML5 Template for new projects.">
  <meta property="og:image" content="image.png">

  <link rel="icon" href="/favicon.ico">
  <link rel="icon" href="/favicon.svg" type="image/svg+xml">
  <link rel="apple-touch-icon" href="/apple-touch-icon.png">

  <link rel="stylesheet" href="css/styles.css?v=1.0">

</head>

<body>
    <p>This is a paragraph</p>
  <script src="js/scripts.js"></script>
</body>
</html>
"#;
